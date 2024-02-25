use super::{ClassicalMatrixEditor, ComplexMatrixEditor};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub enum MatrixEditor {
    #[default]
    None,
    Classical(ClassicalMatrixEditor),
    Complex(ComplexMatrixEditor),
}
