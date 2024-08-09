#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LayoutMenu;

impl LayoutMenu {
    pub fn show(ui: &mut egui::Ui, ctx: &egui::Context, model: &mut Model) {
        egui::menu::menu(ui, "Layout", |ui| {
            ui.label("Tools");
            ui.label("Nodes");
            ui.label("Lines");
            ui.label("Labels");
        });
    }
}
