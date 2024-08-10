use std::{cell::RefCell, rc::Rc};

use egui_plot::PlotPoints;

use crate::{
    canvas_state::CanvasState, line::Line, model::Model, node::Node, position::Position,
    serializable_canvas::SerializableCanvas,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Canvas {
    pub state: CanvasState,
    pub nodes: Vec<Rc<RefCell<Node>>>,
    pub lines: Vec<Line>,
}

impl Canvas {
    pub fn place_on_canvas(&mut self, canvas_details: SerializableCanvas, position: Position) {
        let SerializableCanvas { nodes, lines } = canvas_details;
        let nodes: Vec<Rc<RefCell<Node>>> = nodes
            .iter()
            .map(|n| Rc::new(RefCell::new(n.clone())))
            .collect();
        let lines: Vec<Line> = lines
            .iter()
            .map(|(n1, n2)| (Rc::clone(&nodes[*n1]), Rc::clone(&nodes[*n2])).into())
            .collect();

        let offset = Position::ZERO - position;

        for node in nodes.iter() {
            let mut node = node.borrow_mut();
            node.position += offset;
        }

        self.nodes.extend(nodes);
        self.lines.extend(lines);
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        egui_plot::Plot::new("canvas")
            .data_aspect(1.0)
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                self.draw_lines(plot_ui);
                self.draw_nodes(plot_ui);

                // self.handle_interactions(
                //     plot_ui,
                //     selected_tool,
                //     options,
                //     pointer_coords,
                //     global_pointer_coords,
                // );
            });
    }

    fn draw_lines(&self, plot_ui: &mut egui_plot::PlotUi) {
        for line in &self.lines {
            plot_ui.line(egui_plot::Line::new(line.clone()).color(egui::Color32::LIGHT_BLUE));
        }
    }

    fn draw_nodes(&self, plot_ui: &mut egui_plot::PlotUi) {
        plot_ui.points(egui_plot::Points::new(self.get_nodes_as_points()))
    }

    fn handle_interactions(&mut self, plot_ui: &mut egui_plot::PlotUi, model: &Model) {
        let (pointer_coords, global_pointer_coords) = self.get_pointer_coords(plot_ui);

        // let snap = match options.mode {
        //     Mode::Edit => options.specific.edit.snap,
        //     _ => Snap::None,
        // };

        // TODO
        plot_ui.ctx().input_mut(|state| {
            if plot_ui.response().clicked() {
                match (selected_tool, global_pointer_coords) {
                    (Tool::Move, Some(global_pointer_coords)) => {
                        self.move_node(plot_ui, pointer_coords, global_pointer_coords, snap)
                    }
                    (Tool::Node, _) => {
                        if self.add_node(pointer_coords, snap).is_none() {
                            // TODO normalize errors
                            eprintln!("[{}:{}] Error: Node not created", file!(), line!());
                        }
                    }
                    (Tool::Line, Some(global_pointer_coords)) => {
                        self.add_line(plot_ui, pointer_coords, global_pointer_coords)
                    }
                    (Tool::Label, Some(global_pointer_coords)) => {
                        self.add_label(plot_ui, pointer_coords, global_pointer_coords)
                    }
                    _ => unreachable!(), // TODO add appropriate error message
                }
            }

            self.keypress_handler(plot_ui, state, pointer_coords, global_pointer_coords);
        });
    }

    fn get_pointer_coords(
        &self,
        plot_ui: &egui_plot::PlotUi,
    ) -> (Option<egui_plot::PlotPoint>, Option<egui::Pos2>) {
        (
            plot_ui.pointer_coordinate(),
            plot_ui
                .ctx()
                .input(|i| i.pointer.latest_pos())
                .map(|gpc| gpc - plot_ui.response().drag_delta()),
        )
    }

    fn get_nodes_as_points(&self) -> PlotPoints {
        self.nodes
            .iter()
            .map(|node| node.borrow().position.into())
            .collect()
    }
}

impl serde::Serialize for Canvas {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Canvas {
            nodes: ref_nodes,
            lines: ref_lines,
            ..
        } = self;
        let nodes: Vec<Node> = ref_nodes.iter().map(|n| n.borrow().clone()).collect();
        let lines: Vec<(usize, usize)> = ref_lines
            .iter()
            .map(|Line { start, end }| {
                (
                    ref_nodes.iter().position(|node| *node == *start).unwrap(),
                    ref_nodes.iter().position(|node| *node == *end).unwrap(),
                )
            })
            .collect();
        let canvas_details = SerializableCanvas { nodes, lines };
        canvas_details.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Canvas {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let canvas_details = SerializableCanvas::deserialize(deserializer)?;
        Ok(canvas_details.into())
    }
}

impl From<SerializableCanvas> for Canvas {
    fn from(canvas_details: SerializableCanvas) -> Self {
        let SerializableCanvas { nodes, lines } = canvas_details;
        let nodes: Vec<Rc<RefCell<Node>>> = nodes
            .iter()
            .map(|n| Rc::new(RefCell::new(n.clone())))
            .collect();
        let lines: Vec<Line> = lines
            .iter()
            .map(|(n1, n2)| (Rc::clone(&nodes[*n1]), Rc::clone(&nodes[*n2])).into())
            .collect();
        Self {
            nodes,
            lines,
            ..Default::default()
        }
    }
}
