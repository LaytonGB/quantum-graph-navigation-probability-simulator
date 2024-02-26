use super::{ClassicalMatrixEditor, ComplexMatrixEditor};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub enum MatrixEditor {
    #[default]
    None,
    Classical(ClassicalMatrixEditor),
    Complex(ComplexMatrixEditor),
}

impl MatrixEditor {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_classical(&self) -> bool {
        matches!(self, Self::Classical(_))
    }

    pub fn is_complex(&self) -> bool {
        matches!(self, Self::Complex(_))
    }
}
