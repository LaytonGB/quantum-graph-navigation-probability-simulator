use std::{cell::RefCell, rc::Rc};

use egui::{Color32, FontId, InputState, Key, Modifiers, Pos2, Ui};
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotUi, Points, Text};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::canvas_actions::CanvasActions;
use crate::constants::{NODE_CLICK_PRIORITY_MULTIPLIER, POINTER_INTERACTION_RADIUS};
use crate::context_menu::{ContextMenu, ContextMenuValues};
use crate::graph_line::GraphLine;
use crate::graph_node::GraphNode;
use crate::options::{Mode, Options, Snap};
use crate::tool::Tool;
use crate::utils::euclidean_dist;

pub enum CanvasObject {
    Node(Rc<RefCell<GraphNode>>),
    Line {
        line: GraphLine,
        closest_point_on_line: GraphNode,
    },
}

#[derive(Clone, Default)]
pub struct Canvas {
    pub nodes: Vec<Rc<RefCell<GraphNode>>>,

    pub lines: Vec<GraphLine>,

    pub line_start: Option<Rc<RefCell<GraphNode>>>,

    pub node_being_moved_and_origin: Option<(Rc<RefCell<GraphNode>>, GraphNode)>,

    pub context_menu_values: ContextMenuValues,

    pub action_data: CanvasActions,
}

impl Canvas {
    fn new(nodes: Vec<Rc<RefCell<GraphNode>>>, lines: Option<Vec<(usize, usize)>>) -> Self {
        let lines = lines
            .and_then(|lines| {
                Some(
                    lines
                        .into_iter()
                        .map(|(a, b)| GraphLine::new(nodes[a].clone(), nodes[b].clone()))
                        .collect(),
                )
            })
            .unwrap_or(Vec::new());

        Self {
            nodes,
            lines,
            ..Default::default()
        }
    }

    /// Adds a node to the canvas.
    pub fn add_node(&mut self, node: impl Into<GraphNode>, snap: Snap) -> Result<(), ()> {
        let node: GraphNode = node.into();
        let rounded_node = node.round_to(snap);
        if let Some(rounded_node) = rounded_node {
            if self
                .nodes
                .iter()
                .find(|n| n.borrow().clone() == rounded_node)
                .is_none()
            {
                self.nodes.push(Rc::new(RefCell::new(rounded_node)));
                return Ok(());
            }
        }
        Err(())
    }

    fn move_node(
        &mut self,
        plot_ui: &PlotUi,
        pointer_coords: PlotPoint,
        global_pointer_coords: Pos2,
        snap: Snap,
    ) {
        if let Some((node_being_moved, _)) = self.node_being_moved_and_origin.clone() {
            let node = node_being_moved.borrow().clone();
            if let Some(rounded_node) = node.round_to(snap) {
                if !self
                    .nodes
                    .iter()
                    .any(|n| n != &node_being_moved && n.borrow().clone() == rounded_node)
                {
                    let mut node = node_being_moved.borrow_mut();
                    *node = rounded_node;
                    self.node_being_moved_and_origin = None;
                    return;
                }
            }
            self.reset_moving_node_position();
        } else {
            if let Some((_, node)) = self.find_closest_node_and_dist(pointer_coords) {
                let node_pos = plot_ui.screen_from_plot(node.borrow().clone().into());
                let pointer_to_node_dist = euclidean_dist(&global_pointer_coords, &node_pos);
                if pointer_to_node_dist <= POINTER_INTERACTION_RADIUS {
                    let original_node = node.borrow().clone();
                    self.node_being_moved_and_origin = Some((node, original_node));
                }
            }
        }
    }

    /// Get node closest to given coordinates if a node exists.
    pub fn find_closest_node_and_dist(
        &self,
        coords: impl Into<GraphNode>,
    ) -> Option<(f64, Rc<RefCell<GraphNode>>)> {
        let coords: GraphNode = coords.into();
        self.nodes.iter().fold(None, |closest, node| {
            let dist = euclidean_dist(&node.borrow().clone(), &coords);
            match closest {
                Some((closest_dist, _)) if closest_dist < dist => closest,
                _ => Some((dist, (*node).clone())),
            }
        })
    }

    /// Removes and returns a target node. The node must have the exact same bit
    /// configuration in both x and y floats.
    pub fn remove_node(&mut self, target_node: GraphNode) -> Option<Rc<RefCell<GraphNode>>> {
        let index = self
            .nodes
            .iter()
            .position(|node| &target_node == &node.borrow().clone());
        if let Some(index) = index {
            self.lines = self
                .lines
                .clone()
                .into_iter()
                .filter(|l| !l.is_attatched(&target_node))
                .collect();
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
        self.nodes
            .iter()
            .map(|n| [n.borrow().x, n.borrow().y])
            .collect()
    }

    /// Returns a Points object which stores Point data for presenting the Node
    /// coordinates on the graph.
    pub fn nodes(&self, options: &Options) -> Points {
        Points::new(self.nodes_coords())
            .filled(true)
            .radius(5.0)
            .color(options.get_node_color())
    }

    pub fn draw_nodes(&self, plot_ui: &mut PlotUi, options: &Options) {
        plot_ui.points(self.nodes(options));

        let mut style = (*plot_ui.ctx().style()).clone();
        style
            .text_styles
            .insert(egui::TextStyle::Small, FontId::proportional(26.0));
        plot_ui.ctx().set_style(style);
        for node in &self.nodes {
            plot_ui.text(
                Text::new(
                    node.borrow().clone().into(),
                    node.borrow().label.as_ref().unwrap_or(&"".to_owned()),
                )
                .color(Color32::WHITE),
            );
        }
    }

    pub fn add_line(
        &mut self,
        plot_ui: &PlotUi,
        pointer_coords: PlotPoint,
        global_pointer_coords: Pos2,
    ) {
        if let Some((_, clicked_node)) = self.find_closest_node_and_dist(pointer_coords) {
            let clicked_node_global_pos =
                plot_ui.screen_from_plot(clicked_node.borrow().clone().into());
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

    pub fn add_label(
        &mut self,
        plot_ui: &PlotUi,
        pointer_coords: PlotPoint,
        global_pointer_coords: Pos2,
    ) {
        if let Some((_, label_target)) = self.find_closest_node_and_dist(pointer_coords) {
            let label_target_node = label_target.borrow();
            let target_global_pos =
                plot_ui.screen_from_plot([label_target_node.x, label_target_node.y].into());
            let global_dist = euclidean_dist(&target_global_pos, &global_pointer_coords);
            if global_dist <= POINTER_INTERACTION_RADIUS {
                std::mem::drop(label_target_node);
                label_target.borrow_mut().label = Some(self.action_data.add_label_text.clone());
            }
        }
    }

    pub fn dist_to_line_and_closest_point(&self, p: &GraphNode, l: &GraphLine) -> (GraphNode, f64) {
        let closest_point_on_infinite_line = l.closest_point_to_node(p);
        let (a, b) = (l.start.borrow(), l.end.borrow());
        let (pa, pb) = (p.dist(&a), p.dist(&b));

        // if point `p` is not between endpoint nodes
        // dist (further end -> `p`) > len between endpoints
        let (closer_node, further_dist) = if pa <= pb { (a, pb) } else { (b, pa) };
        if further_dist > l.len() {
            (closer_node.clone(), p.dist(&closer_node))
        } else {
            let dist = p.dist(&closest_point_on_infinite_line);
            (closest_point_on_infinite_line, dist)
        }
    }

    pub fn find_closest_line_and_point_on_line(
        &self,
        pointer_coords: PlotPoint,
    ) -> Option<(f64, GraphNode, GraphLine)> {
        let point: GraphNode = pointer_coords.into();
        self.lines.iter().fold(None, |closest_details, line| {
            let (new_closest, dist) = self.dist_to_line_and_closest_point(&point, line);
            if let Some((closest_dist, closest_node, closest_line)) = closest_details {
                if dist < closest_dist {
                    Some((dist, new_closest, line.clone()))
                } else {
                    Some((closest_dist, closest_node, closest_line))
                }
            } else {
                Some((dist, new_closest, line.clone()))
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

    fn remove_node_if_pointer_within_range(
        &mut self,
        plot_ui: &PlotUi,
        node: Rc<RefCell<GraphNode>>,
        global_pointer_coords: Pos2,
    ) {
        let node_pos = plot_ui.screen_from_plot(node.borrow().clone().into());
        let node_to_pointer_dist = euclidean_dist(&node_pos, &global_pointer_coords);
        if node_to_pointer_dist <= POINTER_INTERACTION_RADIUS {
            self.remove_node(node.borrow().clone());
        }
    }

    fn remove_line_if_pointer_within_range(
        &mut self,
        plot_ui: &PlotUi,
        line: GraphLine,
        point_on_line: GraphNode,
        global_pointer_coords: Pos2,
    ) {
        let line_pos = plot_ui.screen_from_plot(point_on_line.into());
        let line_to_pointer_dist = euclidean_dist(&line_pos, &global_pointer_coords);
        if line_to_pointer_dist <= POINTER_INTERACTION_RADIUS {
            self.remove_line(line);
        }
    }

    fn find_closest_to_pointer(&mut self, pointer_coords: PlotPoint) -> Option<CanvasObject> {
        match (
            self.find_closest_node_and_dist(pointer_coords),
            self.find_closest_line_and_point_on_line(pointer_coords),
        ) {
            (None, Some((_, closest_point_on_line, closest_line))) => Some(CanvasObject::Line {
                line: closest_line,
                closest_point_on_line,
            }),
            (Some((_, closest_node)), None) => Some(CanvasObject::Node(closest_node)),
            (
                Some((closest_node_dist, closest_node)),
                Some((closest_line_dist, closest_point_on_line, closest_line)),
            ) => Some(
                if closest_node_dist / NODE_CLICK_PRIORITY_MULTIPLIER <= closest_line_dist {
                    CanvasObject::Node(closest_node)
                } else {
                    CanvasObject::Line {
                        line: closest_line,
                        closest_point_on_line,
                    }
                },
            ),
            _ => None,
        }
    }

    fn delete_closest_to_pointer(
        &mut self,
        plot_ui: &PlotUi,
        pointer_coords: PlotPoint,
        global_pointer_coords: Result<Pos2, ()>,
    ) {
        if let (Ok(global_pointer_coords), Some(closest)) = (
            global_pointer_coords,
            self.find_closest_to_pointer(pointer_coords),
        ) {
            match closest {
                CanvasObject::Node(node) => {
                    self.remove_node_if_pointer_within_range(plot_ui, node, global_pointer_coords)
                }
                CanvasObject::Line {
                    line,
                    closest_point_on_line,
                } => self.remove_line_if_pointer_within_range(
                    plot_ui,
                    line,
                    closest_point_on_line,
                    global_pointer_coords,
                ),
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
                Key::Escape => self.reset_values(),
                Key::Backspace | Key::Delete
                    if plot_ui.response().hovered() && state.consume_key(Modifiers::NONE, key) =>
                {
                    self.delete_closest_to_pointer(plot_ui, pointer_coords, global_pointer_coords)
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
        snap: Snap,
    ) {
        match (selected_tool, global_pointer_coords) {
            (Tool::Move, Ok(global_pointer_coords)) => {
                self.move_node(plot_ui, pointer_coords, global_pointer_coords, snap)
            }
            (Tool::Node, _) => {
                if self.add_node(pointer_coords, snap).is_err() {
                    // TODO normalize errors
                    eprintln!("[{}:{}] Error: Node not created", file!(), line!());
                }
            }
            (Tool::Line, Ok(global_pointer_coords)) => {
                self.add_line(plot_ui, pointer_coords, global_pointer_coords)
            }
            (Tool::Label, Ok(global_pointer_coords)) => {
                self.add_label(plot_ui, pointer_coords, global_pointer_coords)
            }
            _ => unreachable!(), // TODO add appropriate error message
        }
    }

    fn draw_lines(&self, plot_ui: &mut PlotUi, options: &Options) {
        for line in &self.lines {
            plot_ui.line(Line::new(line.clone()).color(options.get_line_color()));
        }
    }

    fn plot_show(&mut self, plot_ui: &mut PlotUi, selected_tool: Tool, options: &Options) {
        let snap = if options.mode == Mode::Edit {
            options.specific.edit.snap
        } else {
            Snap::None
        };

        self.draw_lines(plot_ui, options);
        self.draw_nodes(plot_ui, options);

        self.reset_values_by_tool(selected_tool);

        let pointer_coords = plot_ui.pointer_coordinate();
        let global_pointer_coords =
            if let Some(global_pointer_coords) = plot_ui.ctx().input(|i| i.pointer.latest_pos()) {
                Ok(global_pointer_coords - plot_ui.response().drag_delta())
            } else {
                Err(())
            };

        if let Some(pointer_coords) = pointer_coords {
            if let Some(start) = &self.line_start {
                let start = start.borrow();
                plot_ui.line(
                    Line::new(vec![
                        [start.x, start.y],
                        [pointer_coords.x, pointer_coords.y],
                    ])
                    .color(Color32::LIGHT_BLUE),
                );
            }

            if let Some((start, _)) = self.node_being_moved_and_origin.clone() {
                let mut start = start.as_ref().borrow_mut();
                start.x = pointer_coords.x;
                start.y = pointer_coords.y;
            }

            plot_ui.ctx().input_mut(|state| {
                if plot_ui.response().clicked() {
                    self.click_handler(
                        selected_tool,
                        plot_ui,
                        pointer_coords,
                        global_pointer_coords,
                        snap,
                    );
                }

                self.keypress_handler(plot_ui, state, pointer_coords, global_pointer_coords);
            });
        }

        plot_ui
            .response()
            .clone()
            .context_menu(|ctx_ui| ContextMenu::plot_context_menu(self, ctx_ui));
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        selected_tool: Tool,
        options: &Options,
        canvas_actions: &CanvasActions,
    ) {
        self.action_data = canvas_actions.clone();
        Plot::new("canvas")
            .data_aspect(1.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                self.plot_show(plot_ui, selected_tool, options)
            });
    }

    fn reset_values_by_tool(&mut self, selected_tool: Tool) {
        if selected_tool != Tool::Line {
            self.line_start = None;
        }

        if selected_tool != Tool::Move {
            self.node_being_moved_and_origin = None;
        }
    }

    fn reset_values(&mut self) {
        self.line_start = None;
        self.reset_moving_node_position();
    }

    fn reset_moving_node_position(&mut self) {
        if let Some((node_being_moved, original_node)) = self.node_being_moved_and_origin.take() {
            let mut node = node_being_moved.borrow_mut();
            *node = original_node;
        }
    }
}

impl Serialize for Canvas {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Canvas", 2)?;
        state.serialize_field("nodes", &self.nodes)?;

        let serializable_lines: Vec<(usize, usize)> = self
            .lines
            .iter()
            .map(|l| {
                self.nodes
                    .iter()
                    .enumerate()
                    .fold((None, None), |(start, end), (i, n)| {
                        if start.is_none() {
                            if l.start == *n {
                                return (Some(i), end);
                            }
                        }
                        if end.is_none() {
                            if l.end == *n {
                                return (start, Some(i));
                            }
                        }
                        (start, end)
                    })
            })
            .filter_map(|(a, b)| {
                if let (Some(a), Some(b)) = (a, b) {
                    Some((a, b))
                } else {
                    None
                }
            })
            .collect();
        state.serialize_field("lines", &serializable_lines)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for Canvas {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CanvasParts {
            nodes: Vec<Rc<RefCell<GraphNode>>>,
            lines: Vec<(usize, usize)>,
        }

        let parts: CanvasParts = Deserialize::deserialize(deserializer)?;

        let canvas = Canvas::new(parts.nodes, Some(parts.lines));

        Ok(canvas)
    }
}
