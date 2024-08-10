use wfd::DialogParams;

use crate::{model::Model, position::Position, EframeApp};

pub struct CanvasMenu;

impl CanvasMenu {
    pub fn show(ui: &mut egui::Ui, model: &mut Model) {
        ui.menu_button("Canvas", |ui| {
            #[cfg(target_family = "windows")]
            ui.menu_button("Add Graph", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Center X:");
                    ui.text_edit_singleline(&mut model.text_fields.add_graph_x);
                });

                ui.horizontal(|ui| {
                    ui.label("Center Y:");
                    ui.text_edit_singleline(&mut model.text_fields.add_graph_y);
                });

                if ui.button("Place graph").clicked() {
                    let path = wfd::open_dialog(DialogParams {
                        file_types: vec![("JSON Files", "*.json")],
                        ..Default::default()
                    })
                    .map(|dialog_result| dialog_result.selected_file_path)
                    .map_err(|e| format!("File load failed:\n{:?}", e));

                    let position = Position::try_from((
                        model.text_fields.add_graph_x,
                        model.text_fields.add_graph_y,
                    ));

                    if let (Ok(path_buffer), Ok(graph_place_coords)) = (path, position) {
                        model.queue_place_graph(path_buffer, graph_place_coords);
                        ui.close_menu();
                    }
                }
            });

            if ui.button("Clear").clicked() {
                model.clear_canvas();
            }
        });
    }
}
