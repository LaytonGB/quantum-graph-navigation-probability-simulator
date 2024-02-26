use anyhow::{anyhow, Error, Result};
use nalgebra::{Complex, DMatrix, DVector};

use super::complex_transition_matrix::ComplexTransitionMatrix;

#[derive(Debug, Clone)]
pub struct ComplexStateManager {
    state: DVector<Complex<f64>>,
    step: usize,
    transition_matrix: ComplexTransitionMatrix,
    start_node_idx: Option<usize>,
}

impl TryFrom<&DMatrix<Complex<f64>>> for ComplexStateManager {
    type Error = Error;

    fn try_from(matrix: &DMatrix<Complex<f64>>) -> Result<Self, Self::Error> {
        match ComplexTransitionMatrix::try_from(matrix) {
            Ok(transition_matrix) => {
                let initial_state = transition_matrix.get_initial_state(&None);
                let mut res = Self {
                    state: initial_state,
                    step: 0,
                    transition_matrix,
                    start_node_idx: None,
                };

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

impl ComplexStateManager {
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
        let nnodes = (self.state.nrows() as f64).sqrt() as usize;
        if nnodes == 0 {
            return DVector::from_element(0, 0.0);
        }

        // sum every nnodes elements to get the state of each node
        // BUG this is not the correct way to sum the elements
        DVector::from_iterator(
            nnodes,
            self.state
                .as_slice()
                .chunks(nnodes)
                .map(|x| x.into_iter().map(|x| x.l1_norm()).sum::<f64>()),
        )
    }

    pub(crate) fn reset_state(&mut self) {
        self.step = 0;
        self.state = self
            .transition_matrix
            .get_initial_state(&self.start_node_idx);
    }

    pub(crate) fn set_transition_matrix_from(&mut self, matrix: &DMatrix<Complex<f64>>) {
        if let Ok(new_transition_matrix) = ComplexTransitionMatrix::try_from(matrix) {
            self.transition_matrix = new_transition_matrix;
        }
    }

    pub fn get_step(&self) -> usize {
        self.step
    }

    pub(crate) fn is_transition_matrix_sized_correctly(&self, nnodes: usize) -> bool {
        nnodes.pow(2) == self.transition_matrix.matrix.ncols()
    }

    pub(crate) fn set_start_node_idx(&mut self, start_node_idx: usize) {
        self.start_node_idx = Some(start_node_idx);
    }
}
