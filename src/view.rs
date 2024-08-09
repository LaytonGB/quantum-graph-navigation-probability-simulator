use crate::{canvas_menu::CanvasMenu, file_menu::FileMenu, layout_menu::LayoutMenu, model::Model};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct View {
    pub title: String,
}

impl View {
    pub fn show(&self, ctx: &egui::Context, model: &mut Model) {
        self.show_top_panel(ctx, model);
        // self.show_left_panel(ctx);
        // self.show_right_panel(ctx);
        // self.show_center_panel(ctx);
    }

    fn show_top_panel(&self, ctx: &egui::Context, model: &mut Model) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // TODO rework this when we can save in web
                #[cfg(not(target_arch = "wasm32"))]
                FileMenu::show(ui, ctx, model);
                ui.add_space(16.0);
                CanvasMenu::show(ui, model);
                ui.add_space(16.0);
                LayoutMenu::show(ui, ctx, model);
                ui.add_space(16.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn show_left_panel(&self, ctx: &egui::Context, model: &mut Model) {
        if self.layout.tools {
            egui::SidePanel::new(Side::Left, "left_panel").show(ctx, |ui| {
                ui.heading("Tools");
                let mut tool_buttons: [Tool; 4] = [Tool::Move, Tool::Node, Tool::Line, Tool::Label];
                for tool in tool_buttons.iter_mut() {
                    tool.show(
                        ui,
                        &mut self.selected_tool,
                        &mut self.canvas_actions.add_label_text,
                    );
                }
                // BUG: this line is needed, allows left-panel resizing
                // is likely fixed if egui is updated
                ui.separator();
            });
        }
    }
}
