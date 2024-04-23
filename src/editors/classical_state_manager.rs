use anyhow::{anyhow, Error, Result};
use nalgebra::{DMatrix, DVector};

use crate::editors::classical_transition_matrix::ClassicalTransitionMatrix;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassicalStateManager {
    state: DVector<f64>,
    step: usize,
    transition_matrix: ClassicalTransitionMatrix,
    start_node_idx: Option<usize>,
}

impl TryFrom<&DMatrix<f64>> for ClassicalStateManager {
    type Error = Error;

    fn try_from(matrix: &DMatrix<f64>) -> Result<Self, Self::Error> {
        match ClassicalTransitionMatrix::try_from(matrix) {
            Ok(transition_matrix) => {
                let initial_state = transition_matrix.get_initial_state(&None);
                let mut res = Self {
                    state: initial_state,
                    step: 0,
                    transition_matrix,
                    start_node_idx: None,
                };

                // TODO implement for reset button also
                // state starts on edge 0,0. this scatters the state to the
                // relevant edges without adding to steps.
                match res.step_forward() {
                    Err(e) => Err(e),
                    Ok(_) => {
                        res.step = 0;
                        Ok(res)
                    }
                }
            }
            Err(e) => Err(anyhow!(e)),
        }
    }
}

impl ClassicalStateManager {
    pub fn step_forward(&mut self) -> Result<()> {
        self.step += 1;
        if let Ok(updated_state) = self.transition_matrix.apply(self.state.clone()) {
            self.state = updated_state;
            Ok(())
        } else {
            Err(anyhow!("Failed to apply transition matrix, try updating the transition matrix from the matrix editor"))
        }
    }

    pub(crate) fn get_state_data(&self) -> DVector<f64> {
        let node_count = (self.state.nrows() as f64).sqrt() as usize;
        if node_count == 0 {
            return DVector::from_element(0, 0.0);
        }

        // sum every node_count elements to get the state of each node
        DVector::from_iterator(
            node_count,
            self.state
                .as_slice()
                .chunks(node_count)
                .map(|x| x.iter().sum::<f64>()),
        )
    }

    pub(crate) fn reset_state(&mut self, matrix: &DMatrix<f64>) {
        self.step = 0;
        if let Ok(new_transition_matrix) = ClassicalTransitionMatrix::try_from(matrix) {
            self.transition_matrix = new_transition_matrix;
        } else {
            panic!("Failed to reset state, could not set transition matrix from matrix editor");
        }
        self.state = self
            .transition_matrix
            .get_initial_state(&self.start_node_idx);
    }

    pub(crate) fn set_transition_matrix_from(&mut self, matrix: &DMatrix<f64>) {
        if let Ok(new_transition_matrix) = ClassicalTransitionMatrix::try_from(matrix) {
            self.transition_matrix = new_transition_matrix;
        }
    }

    pub fn get_step(&self) -> usize {
        self.step
    }

    pub(crate) fn is_transition_matrix_sized_correctly(&self, node_count: usize) -> bool {
        node_count.pow(2) == self.transition_matrix.matrix.ncols()
    }

    pub(crate) fn set_start_node_idx(&mut self, start_node_idx: usize) {
        self.start_node_idx = Some(start_node_idx);
    }
}
