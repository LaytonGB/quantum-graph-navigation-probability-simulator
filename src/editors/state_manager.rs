use crate::editors::{ClassicalStateManager, ComplexStateManager};

#[derive(Debug, Default, Clone)]
pub enum StateManager {
    #[default]
    None,
    Classical(ClassicalStateManager),
    Complex(ComplexStateManager),
}

impl StateManager {
    pub fn show(&mut self, ui: &mut egui::Ui, labels: &[(usize, usize)]) {
        match self {
            Self::Complex(complex_state_manager) => complex_state_manager.show(ui, labels),
            _ => {}
        }
    }
}
