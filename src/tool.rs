use egui::Ui;

// TODO implement enum to string and enum iter crate

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Tool {
    #[default]
    Move,
    Node,
    Line,
    Label,
}

impl Tool {
    pub fn name(&self) -> &'static str {
        match self {
            Tool::Move => "Move",
            Tool::Node => "Node",
            Tool::Line => "Line",
            Tool::Label => "Label",
        }
    }

    pub fn show(&self, ui: &mut Ui, selected_tool: &mut Tool, label_text: &mut String) {
        let mut btn = ui.button(format!("{}", self.name()));
        if selected_tool == self {
            btn = btn.highlight();
            if *self == Tool::Label {
                ui.group(|ui| {
                    ui.label("Label text:");
                    ui.text_edit_singleline(label_text);
                });
            }
        }

        if btn.clicked() {
            *selected_tool = *self;
        }
    }
}
