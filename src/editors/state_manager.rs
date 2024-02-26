use crate::editors::{ClassicalStateManager, ComplexStateManager};

#[derive(Debug, Default, Clone)]
pub enum StateManager {
    #[default]
    None,
    Classical(ClassicalStateManager),
    Complex(ComplexStateManager),
}
