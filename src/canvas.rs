use std::rc::Rc;

use egui::{InputState, Key, Modifiers, Pos2, Ui};
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotUi, Points};
use serde::{Deserialize, Serialize};

use crate::{euclidean_dist, euclidean_squared, GraphLine, GraphNode, Tool};

const POINTER_INTERACTION_RADIUS: f64 = 16.0;

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
    pub fn find_closest_node(&self, coords: impl Into<GraphNode>) -> Option<Rc<GraphNode>> {
        if self.nodes.is_empty() {
            return None;
        }

        let coords: GraphNode = coords.into();
        Some(
            self.nodes
                .iter()
                .fold(None, |closest, node| {
                    let new_closest_distance = euclidean_squared(&**node, &coords);
                    match closest {
                        Some((_, current_distance)) if current_distance < new_closest_distance => {
                            closest
                        }
                        _ => Some((node, new_closest_distance)),
                    }
                })
                .unwrap()
                .0
                .clone(),
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
        if let Some(clicked_node) = self.find_closest_node(pointer_coords) {
            let clicked_node_global_pos = plot_ui.screen_from_plot((*clicked_node).into());
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
                    if let (Ok(global_pointer_coords), Some(counted_node)) = (
                        global_pointer_coords,
                        self.find_closest_node([pointer_coords.x, pointer_coords.y]),
                    ) {
                        let GraphNode {
                            x: node_x,
                            y: node_y,
                        } = *counted_node;
                        let node_pos = plot_ui.screen_from_plot(PlotPoint {
                            x: node_x,
                            y: node_y,
                        });
                        let node_to_pointer_dist =
                            euclidean_dist(&node_pos, &global_pointer_coords);
                        if node_to_pointer_dist <= POINTER_INTERACTION_RADIUS {
                            self.remove_node(GraphNode::new(node_x, node_y));
                        }
                    }
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
            plot_ui.line(Line::new(line.clone()));
        }
    }

    fn plot_show(&mut self, plot_ui: &mut PlotUi, selected_tool: Tool) {
        plot_ui.points(self.nodes());
        self.draw_lines(plot_ui);

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
