use nalgebra::{Complex, DMatrix, DVector, Normed};

use super::complex_transition_matrix::ComplexTransitionMatrix;

#[derive(Debug, Clone)]
pub struct ComplexStateManager {
    state: DVector<Complex<f64>>,
    probability_vector: DVector<f64>,
    is_state_updated: bool,
    step: usize,
    transition_matrix: ComplexTransitionMatrix,
    start_node_idx: Option<usize>,
}

impl ComplexStateManager {
    pub fn new(matrix: &DMatrix<Complex<f64>>, start_node_idx: usize) -> Self {
        let transition_matrix = ComplexTransitionMatrix::new(matrix.clone());

        let initial_state = transition_matrix.get_initial_state(Some(start_node_idx));
        let mut res = Self {
            state: initial_state,
            probability_vector: DVector::from_element(0, 0.0),
            is_state_updated: true,
            step: 0,
            transition_matrix,
            start_node_idx: None,
        };

        // TODO implement for reset button also
        // state starts on edge 0,0. this scatters the state to the
        // relevant edges without adding to steps.
        res.step_forward();
        res.step = 0;
        res
    }

    pub fn step_forward(&mut self) {
        self.step += 1;
        self.state = self.transition_matrix.apply(self.state.clone());
        self.is_state_updated = true;
    }

    pub(crate) fn get_state_data(&mut self) -> DVector<f64> {
        if self.is_state_updated {
            self.update_probability_vector();
            self.is_state_updated = false;
        }
        self.probability_vector.clone()
    }

    pub(crate) fn reset_state(&mut self) {
        self.step = 0;
        self.state = self
            .transition_matrix
            .get_initial_state(self.start_node_idx);
        self.is_state_updated = true;
    }

    pub(crate) fn make_transition_matrix_compatible(&mut self, matrix: &DMatrix<Complex<f64>>) {
        println!("CHECKING TRANSITION MATRIX COMPATIBILITY");
        if self.state.len() != matrix.ncols() || self.probability_vector.len() != matrix.ncols() {
            println!("TRANSITION MATRIX IS NOT COMPATIBLE - UPDATING");
            self.transition_matrix = ComplexTransitionMatrix::new(matrix.clone());
            self.reset_state();
        }
    }

    pub fn get_step(&self) -> usize {
        self.step
    }

    pub(crate) fn set_start_node_idx(&mut self, start_node_idx: usize) {
        self.start_node_idx = Some(start_node_idx);
    }

    pub(crate) fn set_transition_matrix_from(
        &mut self,
        get_combined_matrix: &DMatrix<Complex<f64>>,
    ) {
        self.transition_matrix = ComplexTransitionMatrix::new(get_combined_matrix.clone());
    }

    fn update_probability_vector(&mut self) {
        self.probability_vector = if self.state.nrows() == 0 {
            DVector::from_element(0, 0.0)
        } else {
            // collapse rows
            DVector::from_iterator(
                self.state.nrows(),
                self.state
                    .row_iter()
                    .map(|row| row.iter().map(|x| x.norm()).sum()),
            )
        };
    }
}
