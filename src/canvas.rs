use egui::Ui;
use egui_plot::{Legend, Plot, PlotPoint, Points};
use serde::{Deserialize, Serialize};

use crate::Tool;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Canvas {
    nodes: Vec<[f64; 2]>,
}

impl Canvas {
    pub fn add_node(&mut self, node: impl Into<[f64; 2]>) {
        let node: [f64; 2] = node.into();
        self.nodes.push(node);
    }

    pub fn clear_nodes(&mut self) {
        self.nodes = Vec::new();
    }

    pub fn nodes(&self) -> Points {
        Points::new(self.nodes.clone()).filled(true).radius(5.)
    }

    pub fn show(&mut self, ui: &mut Ui, selected_tool: Tool) {
        let mut pointer_coords = None;
        let plot_res = Plot::new("canvas")
            .data_aspect(1.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.points(self.nodes());

                // TODO examine [`pointer_coordinate`] and reverse to find actual distance mouse-to-node
                pointer_coords = plot_ui.pointer_coordinate();
            });
        let res = plot_res.response;
        if res.clicked() {
            if let Some(PlotPoint { x, y }) = pointer_coords {
                if let Tool::Node = selected_tool {
                    self.add_node((x, y));
                }
            }
        }
    }
}
