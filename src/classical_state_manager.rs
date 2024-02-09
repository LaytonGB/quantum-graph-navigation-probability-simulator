use nalgebra::DMatrix;

use crate::editors::transition_matrix::TransitionMatrix;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ClassicalStateManager {
    matrix: Option<DMatrix<f64>>,
    transition_matrix: Option<TransitionMatrix>,
}

impl TryFrom<DMatrix<f64>> for ClassicalStateManager {
    type Error = &'static str;

    fn try_from(matrix: DMatrix<f64>) -> Result<Self, Self::Error> {
        match TransitionMatrix::try_from(&matrix) {
            Ok(transition_matrix) => Ok(Self {
                matrix: Some(matrix),
                transition_matrix: Some(transition_matrix),
            }),
            Err(e) => Err(e),
        }
    }
}

impl ClassicalStateManager {
    pub fn step_forward(&mut self) {
        if let (Some(matrix), Some(transition_matrix)) = (&mut self.matrix, &self.transition_matrix)
        {
            *matrix = transition_matrix.apply(matrix.clone()).unwrap();
        }
    }
}
