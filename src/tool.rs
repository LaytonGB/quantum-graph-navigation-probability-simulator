use egui::Ui;

pub trait Tool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut Ui, active: bool) -> bool;
}
