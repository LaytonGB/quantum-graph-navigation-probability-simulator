use nalgebra::DMatrix;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ClassicalStateManager {
    matrix: Option<DMatrix<f64>>,
}

impl From<DMatrix<f64>> for ClassicalStateManager {
    fn from(matrix: DMatrix<f64>) -> Self {
        Self {
            matrix: Some(matrix),
        }
    }
}

impl ClassicalStateManager {
    pub fn step_forward(&mut self, transition_matrix: &DMatrix<f64>) {
        if let Some(matrix) = &mut self.matrix {
            *matrix *= transition_matrix;
        }
    }
}
