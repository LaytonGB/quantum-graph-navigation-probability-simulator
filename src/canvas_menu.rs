pub struct CanvasMenu;

impl CanvasMenu {
    pub fn show(ui: &mut egui::Ui) {
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

                #[cfg(target_family = "windows")]
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
}
