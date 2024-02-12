use nalgebra::DMatrix;

use crate::editors::transition_matrix::TransitionMatrix;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassicalStateManager {
    state: DMatrix<f64>,
    transition_matrix: TransitionMatrix,
}

impl TryFrom<&DMatrix<f64>> for ClassicalStateManager {
    type Error = &'static str;

    fn try_from(matrix: &DMatrix<f64>) -> Result<Self, Self::Error> {
        match TransitionMatrix::try_from(matrix) {
            Ok(transition_matrix) => {
                if let Ok(initial_state) = transition_matrix.get_initial_state() {
                    Ok(Self {
                        state: initial_state,
                        transition_matrix,
                    })
                } else {
                    Err("Matrix is all zeros")
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl ClassicalStateManager {
    pub fn step_forward(&mut self) {
        self.state = self.transition_matrix.apply(self.state.clone()).unwrap();
    }
}
