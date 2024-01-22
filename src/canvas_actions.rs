use std::{cell::RefCell, num::ParseFloatError, rc::Rc};

use egui::Ui;

use crate::canvas::Canvas;
use crate::graph_line::GraphLine;
use crate::graph_node::GraphNode;
use crate::{EditorsContainer, EframeApp};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct CanvasActions {
    pub add_label_text: String,
    pub add_graph_values: PlaceGraphValues,
}

impl CanvasActions {
    pub fn canvas_menu(
        &mut self,
        ui: &mut Ui,
        canvas: &mut Canvas,
        editors: &mut EditorsContainer,
    ) {
        ui.menu_button("Canvas", |ui| {
            ui.menu_button("Add Graph", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Center X:");
                    ui.text_edit_singleline(&mut self.add_graph_values.x);
                });

                ui.horizontal(|ui| {
                    ui.label("Center Y:");
                    ui.text_edit_singleline(&mut self.add_graph_values.y);
                });

                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Place graph").clicked() {
                    if let Ok(graph_place_coords) = self.add_graph_values.clone().try_into() {
                        self.place_graph(canvas, graph_place_coords);
                        ui.close_menu();
                    }
                }
            });

            if ui.button("Clear").clicked() {
                canvas.clear_all();
                editors.clear_all();
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn place_graph(&self, canvas: &mut Canvas, graph_center: GraphNode) {
        use wfd::DialogParams;

        let dialog_result = wfd::open_dialog(DialogParams {
            file_types: vec![("JSON Files", "*.json")],
            ..Default::default()
        });
        if let Ok(dialog_result) = dialog_result {
            if let Ok(file) = std::fs::read(dialog_result.selected_file_path) {
                if let Ok(c) = serde_json::from_slice::<EframeApp>(file.as_slice()) {
                    let canvas_details = CanvasDetails::from(c);
                    canvas_details.place_on_canvas(canvas, graph_center);
                }
            }
        }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct PlaceGraphValues {
    pub x: String,
    pub y: String,
}

impl TryInto<GraphNode> for PlaceGraphValues {
    type Error = ParseFloatError;

    fn try_into(self) -> Result<GraphNode, Self::Error> {
        Ok(GraphNode::new_unlabelled(self.x.parse()?, self.y.parse()?))
    }
}

/// Used to store the canvas nodes and lines when placing an existing graph onto
/// the canvas.
struct CanvasDetails {
    pub nodes: Vec<GraphNode>,
    pub lines: Vec<(usize, usize)>,
}

impl CanvasDetails {
    fn place_on_canvas(&self, canvas: &mut Canvas, new_center: GraphNode) {
        let old_len = canvas.nodes.len();
        let old_center = {
            let (x_min, y_min, x_max, y_max) = self.nodes.iter().fold(
                (f64::MAX, f64::MAX, f64::MIN, f64::MIN),
                |(x_min, y_min, x_max, y_max), n| {
                    let GraphNode { x, y, label: _ } = *n;
                    (x.min(x_min), y.min(y_min), x.max(x_max), y.max(y_max))
                },
            );
            // TODO add float divisibility
            GraphNode::new_unlabelled(x_max + x_min, y_max + y_min)
                / GraphNode::new_unlabelled(2.0, 2.0)
        };
        let center_translation = new_center - old_center;
        for node in &self.nodes {
            let node = node.clone() + center_translation.clone(); // TODO tidy cloning here
            canvas.nodes.push(Rc::new(RefCell::new(node)));
        }
        for (start_idx, end_idx) in &self.lines {
            canvas.lines.push(GraphLine::new(
                canvas.nodes[start_idx + old_len].clone(),
                canvas.nodes[end_idx + old_len].clone(),
            ));
        }
    }
}

impl From<EframeApp> for CanvasDetails {
    fn from(value: EframeApp) -> Self {
        let nodes: Vec<GraphNode> = value
            .canvas
            .nodes
            .iter()
            .map(|n| n.borrow().clone())
            .collect();
        let lines: Vec<(usize, usize)> = value
            .canvas
            .lines
            .iter()
            .filter_map(|l| {
                let (a, b) = (l.start.borrow().clone(), l.end.borrow().clone());
                let (a, b) = (
                    nodes.iter().position(|n| *n == a),
                    nodes.iter().position(|n| *n == b),
                );
                if let (Some(a), Some(b)) = (a, b) {
                    Some((a, b))
                } else {
                    None
                }
            })
            .collect();
        Self { nodes, lines }
    }
}
