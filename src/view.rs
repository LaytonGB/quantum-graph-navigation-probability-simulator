use crate::{canvas_menu::CanvasMenu, file_menu::FileMenu};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct View {
    pub title: String,
}

impl View {
    pub fn show(&self, ctx: &egui::Context) {
        self.show_top_panel(ctx);
        // self.show_left_panel(ctx);
        // self.show_right_panel(ctx);
        // self.show_center_panel(ctx);
    }

    fn show_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // TODO rework this when we can save in web
                #[cfg(not(target_arch = "wasm32"))]
                FileMenu::show(ui);
                CanvasMenu::show(ui);
                ui.add_space(16.0);

                ui.menu_button("Layout", |ui| {
                    ui.checkbox(&mut self.layout.tools, "Tools");
                    ui.checkbox(&mut self.layout.mode, "Modes");
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }
}
