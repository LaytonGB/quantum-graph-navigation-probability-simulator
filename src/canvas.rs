use std::rc::Rc;

use egui::{Color32, InputState, Key, Modifiers, Pos2, Ui};
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotUi, Points};
use serde::{Deserialize, Serialize};

use crate::{
    euclidean_dist, euclidean_squared, GraphLine, GraphNode, Tool, NODE_CLICK_PRIORITY_MULTIPLIER,
    POINTER_INTERACTION_RADIUS,
};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Canvas {
    nodes: Vec<Rc<GraphNode>>,

    lines: Vec<GraphLine>,

    #[serde(skip)]
    line_start: Option<Rc<GraphNode>>,
}

impl Canvas {
    /// Adds a node to the canvas.
    pub fn add_node(&mut self, node: impl Into<GraphNode>) {
        self.nodes.push(Rc::new(node.into()));
    }

    /// Get node closest to given coordinates if a node exists.
    pub fn find_closest_node_and_dist(
        &self,
        coords: impl Into<GraphNode>,
    ) -> Option<(f64, Rc<GraphNode>)> {
        if self.nodes.is_empty() {
            return None;
        }

        let coords: GraphNode = coords.into();
        Some(
            self.nodes
                .iter()
                .fold(None, |closest, node| {
                    let dist = euclidean_squared(&**node, &coords);
                    match closest {
                        Some((closest_dist, _)) if closest_dist < dist => closest,
                        _ => Some((dist, (*node).clone())),
                    }
                })
                .unwrap(),
        )
    }

    /// Removes and returns a target node. The node must have the exact same bit
    /// configuration in both x and y floats.
    pub fn remove_node(&mut self, target_node: GraphNode) -> Option<Rc<GraphNode>> {
        let index = self.nodes.iter().position(|node| target_node == **node);
        if let Some(index) = index {
            Some(self.nodes.remove(index))
        } else {
            None
        }
    }

    /// Removes all nodes.
    pub fn clear_nodes(&mut self) {
        self.nodes = Vec::new();
    }

    pub fn clear_lines(&mut self) {
        self.lines = Vec::new();
    }

    pub fn clear_all(&mut self) {
        self.clear_lines();
        self.clear_nodes();
    }

    /// Returns all nodes as tuple slices.
    fn nodes_coords(&self) -> Vec<[f64; 2]> {
        self.nodes.iter().map(|n| [n.x, n.y]).collect()
    }

    /// Returns a Points object which stores Point data for presenting the Node
    /// coordinates on the graph.
    pub fn nodes(&self) -> Points {
        Points::new(self.nodes_coords()).filled(true).radius(5.)
    }

    pub fn add_line(
        &mut self,
        plot_ui: &PlotUi,
        pointer_coords: PlotPoint,
        global_pointer_coords: Pos2,
    ) {
        if let Some((_, clicked_node)) = self.find_closest_node_and_dist(pointer_coords) {
            let clicked_node_global_pos = plot_ui.screen_from_plot((*clicked_node).clone().into());
            if euclidean_dist(&clicked_node_global_pos, &global_pointer_coords.into())
                <= POINTER_INTERACTION_RADIUS
            {
                if let Some(start_node) = &self.line_start {
                    let line = GraphLine::new(start_node.clone(), clicked_node);
                    if line.start != line.end && self.lines.iter().find(|l| **l == line).is_none() {
                        self.line_start = None;
                        self.lines.push(line);
                    }
                } else {
                    self.line_start = Some(clicked_node);
                }
            }
        }
    }

    pub fn find_closest_line_and_point_on_line(
        &self,
        pointer_coords: PlotPoint,
    ) -> Option<(f64, GraphNode, GraphLine)> {
        let point: GraphNode = pointer_coords.into();
        self.lines.iter().fold(None, |closest_details, line| {
            let intersection_point = dbg!(line.closest_point_to_node(&point));
            let dist = dbg!(intersection_point.dist(&point));
            if let Some((closest_dist, closest_point, closest_line)) = closest_details {
                // TODO see if `is_nan` is necessary
                if !dist.is_nan() && dist < closest_dist {
                    Some((dist, intersection_point, line.clone()))
                } else {
                    Some((closest_dist, closest_point, closest_line))
                }
            } else {
                Some((dist, intersection_point, line.clone()))
            }
        })
    }

    pub fn remove_line(&mut self, target_line: GraphLine) -> Option<GraphLine> {
        let index = self.lines.iter().position(|l| target_line == *l);
        if let Some(index) = index {
            Some(self.lines.remove(index))
        } else {
            None
        }
    }

    fn force_remove_node(
        &mut self,
        plot_ui: &PlotUi,
        node: Rc<GraphNode>,
        global_pointer_coords: Pos2,
    ) {
        let node_pos = plot_ui.screen_from_plot((*node).clone().into());
        let node_to_pointer_dist = euclidean_dist(&node_pos, &global_pointer_coords);
        if node_to_pointer_dist <= POINTER_INTERACTION_RADIUS {
            self.remove_node((*node).clone().into());
        }
    }

    fn force_remove_line(
        &mut self,
        plot_ui: &PlotUi,
        line: GraphLine,
        point_on_line: GraphNode,
        global_pointer_coords: Pos2,
    ) {
        let line_pos = dbg!(plot_ui.screen_from_plot(point_on_line.into()));
        let line_to_pointer_dist = dbg!(euclidean_dist(&line_pos, &global_pointer_coords));
        if line_to_pointer_dist <= POINTER_INTERACTION_RADIUS {
            self.remove_line(line);
        }
    }

    fn delete_operation(
        &mut self,
        plot_ui: &PlotUi,
        pointer_coords: PlotPoint,
        global_pointer_coords: Result<Pos2, ()>,
    ) {
        match (
            global_pointer_coords,
            self.find_closest_node_and_dist(pointer_coords),
            dbg!(self.find_closest_line_and_point_on_line(pointer_coords)),
        ) {
            (Ok(global_pointer_coords), None, Some((_, closest_point_on_line, closest_line))) => {
                self.force_remove_line(
                    plot_ui,
                    closest_line,
                    closest_point_on_line,
                    global_pointer_coords,
                )
            }
            (Ok(global_pointer_coords), Some((_, closest_node)), None) => {
                self.force_remove_node(plot_ui, closest_node, global_pointer_coords)
            }
            (
                Ok(global_pointer_coords),
                Some((closest_node_dist, closest_node)),
                Some((closest_line_dist, closest_point_on_line, closest_line)),
            ) => {
                if closest_node_dist / NODE_CLICK_PRIORITY_MULTIPLIER <= closest_line_dist {
                    self.force_remove_node(plot_ui, closest_node, global_pointer_coords);
                } else {
                    self.force_remove_line(
                        plot_ui,
                        closest_line,
                        closest_point_on_line,
                        global_pointer_coords,
                    )
                }
            }
            _ => (),
        };
    }

    /// Consumes keypress data to perform graph interactions.
    fn keypress_handler(
        &mut self,
        plot_ui: &PlotUi,
        state: &mut InputState,
        pointer_coords: PlotPoint,
        global_pointer_coords: Result<Pos2, ()>,
    ) {
        for key in state.keys_down.clone() {
            match key {
                Key::Backspace | Key::Delete
                    if plot_ui.response().hovered() && state.consume_key(Modifiers::NONE, key) =>
                {
                    self.delete_operation(plot_ui, pointer_coords, global_pointer_coords)
                }
                _ => (),
            }
        }
    }

    fn click_handler(
        &mut self,
        selected_tool: Tool,
        plot_ui: &PlotUi,
        pointer_coords: PlotPoint,
        global_pointer_coords: Result<Pos2, ()>,
    ) {
        match (selected_tool, global_pointer_coords) {
            (Tool::Select, _) => (),
            (Tool::Node, _) => self.add_node(pointer_coords),
            (Tool::Line, Ok(global_pointer_coords)) => {
                self.add_line(plot_ui, pointer_coords, global_pointer_coords)
            }
            _ => (), // TODO add appropriate error message
        }
    }

    fn plot_context_menu(&self, ctx_ui: &mut Ui) {
        if ctx_ui.button("Close this menu").clicked() {
            ctx_ui.close_menu();
        }
    }

    fn draw_lines(&self, plot_ui: &mut PlotUi) {
        for line in &self.lines {
            plot_ui.line(Line::new(line.clone()).color(Color32::BLUE));
        }
    }

    fn plot_show(&mut self, plot_ui: &mut PlotUi, selected_tool: Tool) {
        self.draw_lines(plot_ui);
        plot_ui.points(self.nodes());

        let pointer_coords = plot_ui.pointer_coordinate();
        let global_pointer_coords =
            if let Some(global_pointer_coords) = plot_ui.ctx().input(|i| i.pointer.latest_pos()) {
                Ok(global_pointer_coords - plot_ui.response().drag_delta())
            } else {
                Err(())
            };

        if let Some(pointer_coords) = pointer_coords {
            plot_ui.ctx().input_mut(|state| {
                if plot_ui.response().clicked() {
                    self.click_handler(
                        selected_tool,
                        plot_ui,
                        pointer_coords,
                        global_pointer_coords,
                    );
                }

                self.keypress_handler(plot_ui, state, pointer_coords, global_pointer_coords);
            });
        }

        plot_ui
            .response()
            .clone()
            .context_menu(|ctx_ui| self.plot_context_menu(ctx_ui));
    }

    pub fn show(&mut self, ui: &mut Ui, selected_tool: Tool) {
        Plot::new("canvas")
            .data_aspect(1.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| self.plot_show(plot_ui, selected_tool));
    }
}
