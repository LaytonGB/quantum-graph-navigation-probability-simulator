use egui::{InputState, Key, Modifiers, Pos2, Ui};
use egui_plot::{Legend, Plot, PlotPoint, PlotUi, Points};
use serde::{Deserialize, Serialize};

use crate::Tool;

const POINTER_INTERACTION_RADIUS: f64 = 16.0;

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

    fn key_handler(
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
                    if let (Ok(Pos2 { x, y }), Ok([node_x, node_y])) = (
                        global_pointer_coords,
                        self.find_closest_node([pointer_coords.x, pointer_coords.y]),
                    ) {
                        let node_pos = plot_ui.screen_from_plot(PlotPoint {
                            x: node_x,
                            y: node_y,
                        });
                        let node_to_pointer_dist =
                            ((node_pos.x - x).powi(2) + (node_pos.y - y).powi(2)).sqrt();
                        if node_to_pointer_dist as f64 <= POINTER_INTERACTION_RADIUS {
                            self.remove_node([node_x, node_y]).ok();
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn click_handler(
        &mut self,
        plot_ui: &PlotUi,
        state: &mut InputState,
        selected_tool: Tool,
        pointer_coords: PlotPoint,
        global_pointer_coords: Result<Pos2, ()>,
    ) {
        match selected_tool {
            Tool::Select => (),
            Tool::Node => self.add_node((pointer_coords.x, pointer_coords.y)),
            Tool::Line => todo!(),
        }
    }

    fn plot_show(&mut self, plot_ui: &mut PlotUi, selected_tool: Tool) {
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
                        plot_ui,
                        state,
                        selected_tool,
                        pointer_coords,
                        global_pointer_coords,
                    );
                }

                self.key_handler(plot_ui, state, pointer_coords, global_pointer_coords);
            });
        }
    }

    pub fn show(&mut self, ui: &mut Ui, selected_tool: Tool) {
        Plot::new("canvas")
            .data_aspect(1.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| self.plot_show(plot_ui, selected_tool));
    }
}
