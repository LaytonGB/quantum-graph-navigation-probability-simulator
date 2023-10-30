use egui::Ui;
use serde::{Deserialize, Serialize};

// TODO implement enum to string and enum iter crate

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tool {
    #[default]
    Move,
    Node,
    Line,
}

impl Tool {
    pub fn name(&self) -> &'static str {
        match self {
            Tool::Move => "Move",
            Tool::Node => "Node",
            Tool::Line => "Line",
        }
    }

    pub fn show(&self, ui: &mut Ui, selected_tool: &mut Tool) {
        let mut btn = ui.button(format!("{}", self.name()));
        if selected_tool == self {
            btn = btn.highlight();
        }

        if btn.clicked() {
            *selected_tool = *self;
        }
    }
}
