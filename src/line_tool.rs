use egui::Ui;

use crate::Tool;

pub struct LineTool {
    name: &'static str,
}

impl Default for LineTool {
    fn default() -> Self {
        Self { name: "Line" }
    }
}

impl Tool for LineTool {
    fn name(&self) -> &'static str {
        self.name
    }

    fn show(&mut self, ui: &mut Ui, active: bool) -> bool {
        let mut btn = ui.button(format!("{}", self.name).to_owned());
        if active {
            btn = btn.highlight();
        }
        if btn.clicked() {
            true
        } else {
            false
        }
    }
}
