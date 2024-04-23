use std::collections::HashMap;

use crate::{
    editors::{ClassicalStateManager, ComplexStateManager},
    options::{Mode, Options},
};

#[derive(Debug, Default, Clone)]
pub enum StateManager {
    #[default]
    None,
    Classical(Box<ClassicalStateManager>),
    Complex(Box<ComplexStateManager>),
}

impl StateManager {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        options: &Options,
        adjacency_list: &HashMap<usize, Vec<usize>>,
        labels: &[(usize, usize)],
    ) {
        if let (Self::Complex(csm), Mode::Quantum) = (self, options.mode) {
            csm.show(ui, adjacency_list, labels)
        }
    }
}
