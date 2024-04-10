use std::collections::HashMap;

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
        let res = Self {
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
        // res.step_forward(); // NOTE complex state steps forwards on program open when enabled
        // res.step = 0;
        res
    }

    pub fn step_forward(&mut self) {
        self.step += 1;
        self.state = self.transition_matrix.apply(self.state.clone());
        self.is_state_updated = true;
    }

    pub(crate) fn get_state_data(
        &mut self,
        adjacency_list: &HashMap<usize, Vec<usize>>,
    ) -> DVector<f64> {
        if self.is_state_updated {
            self.update_probability_vector(adjacency_list);
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
        // TODO add a check for the probability vector
        if self.state.len() != matrix.ncols()
        /*  || self.probability_vector.len() != matrix.ncols() */
        {
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

    fn update_probability_vector(&mut self, adjacency_list: &HashMap<usize, Vec<usize>>) {
        self.probability_vector = if self.state.nrows() == 0 {
            DVector::from_element(0, 0.0)
        } else {
            // collapse rows
            let temp = DVector::from_iterator(
                self.state.nrows(),
                self.state
                    .row_iter()
                    .map(|row| row.iter().map(|x| x.norm_squared()).sum::<f64>()),
            );

            // collapse adjacent columns
            let mut res = DVector::from_element(adjacency_list.len(), 0.0);
            let mut past_edges = 0;
            for i in 0..adjacency_list.len() {
                let edge_count = adjacency_list.get(&i).unwrap().len();
                for j in 0..edge_count {
                    res[i] += temp[past_edges + j];
                }
                past_edges += edge_count;
            }
            res
        };
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        labels: &[(usize, usize)],
        adjacency_list: &HashMap<usize, Vec<usize>>,
    ) {
        self.transition_matrix.show(ui, labels);

        ui.separator();

        ui.heading("State");
        ui.collapsing("Complex", |ui| {
            self.display_half_edge_vector(ui, &self.state, labels);
        });
        let probability_vector = self.get_state_data(adjacency_list);
        ui.collapsing("As Probabilities", |ui| {
            self.display_node_vector(ui, &probability_vector, labels)
        });

        ui.separator();

        ui.heading("State Debug Info");
        self.show_debug_info(ui);

        ui.separator();
    }

    fn display_half_edge_vector(
        &self,
        ui: &mut egui::Ui,
        vector: &DVector<Complex<f64>>,
        labels: &[(usize, usize)],
    ) {
        if labels.len() != vector.nrows() {
            panic!("Matrix dimensions do not match labels")
        }

        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::Grid::new("matrix_preview")
                .striped(true)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    // column headers
                    for l in labels.iter() {
                        ui.label(egui::RichText::new(format!("{}->{}", l.0, l.1)).strong());
                    }
                    ui.end_row();

                    // values
                    for i in 0..labels.len() {
                        if i >= vector.len() {
                            break;
                        }

                        if vector[i].l1_norm() == 0.0 {
                            ui.label("-");
                        } else {
                            ui.label(format!("{:.03}", vector[i]));
                        }
                    }
                });
        });
    }

    fn display_node_vector(
        &self,
        ui: &mut egui::Ui,
        vector: &DVector<f64>,
        labels: &[(usize, usize)],
    ) {
        let mut labels = labels.iter().map(|x| x.0).collect::<Vec<_>>();
        labels.dedup();
        if labels.len() != vector.nrows() {
            panic!("Matrix dimensions do not match labels")
        }

        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::Grid::new("matrix_preview")
                .striped(true)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    // column headers
                    for l in labels.iter() {
                        ui.label(egui::RichText::new(format!("{}", l)).strong());
                    }
                    ui.end_row();

                    // values
                    for i in 0..labels.len() {
                        if vector[i] < 0.001 {
                            ui.label("-");
                        } else {
                            ui.label(format!("{:.03}", vector[i]));
                        }
                    }
                });
        });
    }

    fn show_debug_info(&self, ui: &mut egui::Ui) {
        let probability_sum = self.probability_vector.iter().sum::<f64>();
        ui.label(format!(
            "Probabilities Sum: {:.03}",
            probability_sum
        ));
    }
}
