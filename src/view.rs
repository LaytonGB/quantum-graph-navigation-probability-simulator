use egui::panel::Side;
use strum::IntoEnumIterator;

use crate::{canvas_menu::CanvasMenu, file_menu::FileMenu, model::Model, state::State, tool::Tool};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct View;

impl View {
    pub fn show(ctx: &egui::Context, model: &mut Model) {
        Self::show_top_panel(ctx, model);
        Self::show_left_panel(ctx, model);
        // self.show_right_panel(ctx);
        // self.show_center_panel(ctx);
    }

    fn show_top_panel(ctx: &egui::Context, model: &mut Model) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // TODO rework this when we can save in web
                #[cfg(not(target_arch = "wasm32"))]
                {
                    FileMenu::show(ui, ctx, model);
                    ui.add_space(16.0);
                }
                CanvasMenu::show(ui, model);
                ui.add_space(16.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn show_left_panel(ctx: &egui::Context, model: &mut Model) {
        if let (State::Editing { selected_tool }, true) = (&mut model.state, model.panels.tools) {
            egui::SidePanel::new(Side::Left, "left_panel").show(ctx, |ui| {
                ui.heading("Tools");
                for tool in Tool::iter() {
                    tool.show(ui, selected_tool);
                }
                // BUG: this line is needed, allows left-panel resizing
                // is likely fixed if egui is updated
                ui.separator();
            });
        }
    }
}
