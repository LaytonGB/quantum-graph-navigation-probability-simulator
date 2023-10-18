use egui::{Pos2, Ui};
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

    pub fn find_closest_node(&mut self, coords: impl Into<[f64; 2]>) -> Result<[f64; 2], ()> {
        if self.nodes.is_empty() {
            return Err(());
        }

        let coords: [f64; 2] = coords.into();
        Ok(self
            .nodes
            .iter()
            .fold(None, |closest, node| {
                let dist = ((node[0] - coords[0]).powi(2) + (node[1] - coords[1]).powi(2)).sqrt();
                match closest {
                    Some((_, closest_dist)) if closest_dist <= dist => closest,
                    _ => Some((node, dist)),
                }
            })
            .unwrap()
            .0
            .to_owned())
    }

    pub fn remove_node(&mut self, target_node: [f64; 2]) -> Result<(), ()> {
        let index = self.nodes.iter().position(|&node| target_node == node);
        if let Some(index) = index {
            self.nodes.remove(index);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn clear_nodes(&mut self) {
        self.nodes = Vec::new();
    }

    pub fn nodes(&self) -> Points {
        Points::new(self.nodes.clone()).filled(true).radius(5.)
    }

    // TODO cleanup
    pub fn show(&mut self, ui: &mut Ui, selected_tool: Tool) {
        let mut pointer_coords = None;
        let mut global_pointer_coords = Err(());

        Plot::new("canvas")
            .data_aspect(1.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.points(self.nodes());

                let res = plot_ui.response();
                pointer_coords = plot_ui.pointer_coordinate();
                global_pointer_coords = if let Some(global_pointer_coords) =
                    plot_ui.ctx().input(|i| i.pointer.latest_pos())
                {
                    Ok(global_pointer_coords - res.drag_delta())
                } else {
                    Err(())
                };

                if res.clicked() {
                    if let Some(PlotPoint {
                        x: pointer_x,
                        y: pointer_y,
                    }) = pointer_coords
                    {
                        match selected_tool {
                            Tool::Select => {
                                let _ = dbg!(global_pointer_coords);
                                let node = dbg!(self.find_closest_node([pointer_x, pointer_y]));
                                match (global_pointer_coords, node) {
                                    (
                                        Ok(Pos2 {
                                            x: global_x,
                                            y: global_y,
                                        }),
                                        Ok([node_x, node_y]),
                                    ) => {
                                        let node_pos = dbg!(plot_ui.screen_from_plot(PlotPoint {
                                            x: node_x,
                                            y: node_y,
                                        }));
                                        let node_to_pointer_dist = ((node_pos.x - global_x)
                                            .powi(2)
                                            + (node_pos.y - global_y).powi(2))
                                        .sqrt();
                                        if node_to_pointer_dist <= 16.0 {
                                            let _ = dbg!(self.remove_node([node_x, node_y]));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            Tool::Node => self.add_node((pointer_x, pointer_y)),
                            Tool::Line => todo!(),
                        }
                    }
                }
            });
    }
}
