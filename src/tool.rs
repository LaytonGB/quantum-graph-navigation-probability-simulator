use egui::Ui;

// TODO make some kind of tool enum

pub trait Tool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut Ui, active: bool) -> bool;
}
