pub fn show_file_menu(ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| self.show_file_menu(ui, ctx));
    ui.add_space(16.0);
}

pub fn show_canvas_menu(ui: &mut egui::Ui) {
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

pub fn show_layout_menu(ui: &mut egui::Ui) {
    ui.menu_button("Layout", |ui| {
        ui.checkbox(&mut self.layout.tools, "Tools");
        ui.checkbox(&mut self.layout.mode, "Modes");
    });
}
