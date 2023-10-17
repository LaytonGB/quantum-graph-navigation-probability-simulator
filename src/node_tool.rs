use egui::Ui;

use crate::Tool;

pub struct NodeTool {
    name: &'static str,
}

impl Default for NodeTool {
    fn default() -> Self {
        Self { name: "Node" }
    }
}

impl Tool for NodeTool {
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
