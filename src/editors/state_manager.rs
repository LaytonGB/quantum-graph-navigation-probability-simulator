use std::collections::HashMap;

use crate::{
    editors::{ClassicalStateManager, ComplexStateManager},
    options::{Mode, Options},
};

#[derive(Debug, Default, Clone)]
pub enum StateManager {
    #[default]
    None,
    Classical(ClassicalStateManager),
    Complex(ComplexStateManager),
}

impl StateManager {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        options: &Options,
        adjacency_list: &HashMap<usize, Vec<usize>>,
        labels: &[(usize, usize)],
    ) {
        match (self, options.mode) {
            (Self::Complex(complex_state_manager), Mode::Quantum) => {
                complex_state_manager.show(ui, adjacency_list, labels)
            }
            _ => {}
        }
    }
}
