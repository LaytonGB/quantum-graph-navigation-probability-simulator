use nalgebra::{DMatrix, DVector};

use crate::editors::transition_matrix::TransitionMatrix;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassicalStateManager {
    state: DVector<f64>,
    step: usize,
    transition_matrix: TransitionMatrix,
}

impl TryFrom<&DMatrix<f64>> for ClassicalStateManager {
    type Error = &'static str;

    fn try_from(matrix: &DMatrix<f64>) -> Result<Self, Self::Error> {
        match TransitionMatrix::try_from(matrix) {
            Ok(transition_matrix) => {
                let initial_state = transition_matrix.get_initial_state();
                Ok(Self {
                    state: initial_state,
                    step: 0,
                    transition_matrix,
                })
            }
            Err(e) => Err(e),
        }
    }
}

impl ClassicalStateManager {
    pub fn step_forward(&mut self) {
        self.step += 1;
        self.state = self.transition_matrix.apply(self.state.clone()).unwrap();
    }

    pub(crate) fn get_state_data(&self) -> DVector<f64> {
        let nnodes = (self.state.nrows() as f64).sqrt() as usize;
        if nnodes == 0 {
            return DVector::from_element(0, 0.0);
        }
        DVector::from_iterator(
            nnodes,
            self.state
                .as_slice()
                .chunks(nnodes)
                .map(|x| x.into_iter().sum::<f64>()),
        )
    }

    pub(crate) fn reset_state(&mut self) {
        self.step = 0;
        self.state = self.transition_matrix.get_initial_state();
    }

    pub(crate) fn set_transition_matrix_from(&mut self, matrix: &DMatrix<f64>) {
        if let Ok(new_transition_matrix) = TransitionMatrix::try_from(matrix) {
            self.transition_matrix = new_transition_matrix;
        }
    }
}
