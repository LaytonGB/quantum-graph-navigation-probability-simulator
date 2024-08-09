use crate::{model, position::Position};

pub struct CanvasMenu;

impl CanvasMenu {
    pub fn show(ui: &mut egui::Ui, model: &mut model::Model) {
        ui.menu_button("Canvas", |ui| {
            ui.menu_button("Add Graph", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Center X:");
                    ui.text_edit_singleline(&mut model.text_fields.add_graph_x);
                });

                ui.horizontal(|ui| {
                    ui.label("Center Y:");
                    ui.text_edit_singleline(&mut model.text_fields.add_graph_y);
                });

                #[cfg(target_family = "windows")]
                if ui.button("Place graph").clicked() {
                    if let Ok(graph_place_coords) = Position::try_from((
                        model.text_fields.add_graph_x,
                        model.text_fields.add_graph_y,
                    )) {
                        model.canvas.place_graph(graph_place_coords);
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
}
