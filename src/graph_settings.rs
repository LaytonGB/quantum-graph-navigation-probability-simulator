use egui::Ui;

use crate::Canvas;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct CanvasSettings;

impl CanvasSettings {
    pub fn canvas_menu(ui: &mut Ui, canvas: &mut Canvas) {
        ui.menu_button("Canvas", |ui| {
            if ui.button("Clear").clicked() {
                canvas.clear_all();
            }
        });
    }
}
