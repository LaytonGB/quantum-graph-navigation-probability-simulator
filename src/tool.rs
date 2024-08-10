#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    strum::Display,
    strum::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Tool {
    #[default]
    Move,
    Node,
    Line,
    Label,
}

impl Tool {
    pub fn show(&self, ui: &mut egui::Ui, selected_tool: &mut Tool) {
        let mut btn = ui.button(self.to_string());
        if selected_tool == self {
            btn = btn.highlight();
        }

        if btn.clicked() {
            *selected_tool = *self;
        }
    }
}
