use egui::Ui;

use crate::Canvas;

#[derive(Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, Debug)]
pub enum Snap {
    None,
    Half,
    #[default]
    One,
    Five,
    Ten,
}

impl Snap {
    fn as_num_str(&self) -> String {
        match self {
            Snap::None => "None",
            Snap::Half => "0.5",
            Snap::One => "1.0",
            Snap::Five => "5.0",
            Snap::Ten => "10.0",
        }
        .to_owned()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct CanvasSettings {
    pub snap: Snap,
}

impl CanvasSettings {
    fn draw_snap_options(&mut self, ui: &mut Ui) {
        ui.collapsing("Snapping Increment", |ui| {
            ui.horizontal(|ui| {
                for snap in [Snap::None, Snap::Half, Snap::One, Snap::Five, Snap::Ten] {
                    ui.radio_value(&mut self.snap, snap, snap.as_num_str());
                }
            });
        });
    }

    pub fn show(&mut self, ui: &mut Ui, canvas: &mut Canvas) {
        ui.menu_button("Canvas", |ui| {
            self.draw_snap_options(ui);

            if ui.button("Clear").clicked() {
                canvas.clear_all();
            }

            ui.shrink_width_to_current();
        });
    }
}
