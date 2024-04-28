use egui::Color32;
use nalgebra::{Complex, DMatrix, DVector};

use super::transition_matrix_correction_type::TransitionMatrixCorrectionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Headers {
    Show,
    Hide,
}

#[derive(Debug, Clone)]
pub struct ComplexTransitionMatrix {
    matrix: DMatrix<Complex<f64>>,
    last_normalization_correction: TransitionMatrixCorrectionType,
    max_error: f64,
}

impl std::fmt::Display for ComplexTransitionMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl ComplexTransitionMatrix {
    pub fn new(matrix: DMatrix<Complex<f64>>) -> Self {
        let mut res = Self {
            matrix,
            last_normalization_correction: TransitionMatrixCorrectionType::None,
            max_error: 1e-10,
        };
        res.normalize_unitary();
        res
    }

    pub(crate) fn show(&self, ui: &mut egui::Ui, labels: &[(usize, usize)]) {
        ui.heading("Transition Matrix Data");

        ui.collapsing("Unitary Normalized Transition Matrix", |ui| {
            self.display_matrix(ui, labels);
        });

        let error_color = Color32::from_rgb(255, 50, 50);
        match &self.last_normalization_correction {
            TransitionMatrixCorrectionType::None => {
                ui.label("No error correction applied.");
            }
            TransitionMatrixCorrectionType::Scalar(x) => {
                ui.horizontal(|ui| {
                    ui.colored_label(error_color, "Scalar correction applied:".to_string());
                    ui.label(egui::RichText::new(format!("{:.03}", x)).strong());
                });
            }
            TransitionMatrixCorrectionType::NonScalar(correction_vector) => {
                ui.collapsing(
                    egui::RichText::new("Non scalar correction applied").color(error_color),
                    |ui| Self::display_vector(ui, correction_vector, labels, Headers::Hide),
                );
            }
        }
    }

    pub fn get_complex_matrix(&self) -> &DMatrix<Complex<f64>> {
        &self.matrix
    }

    pub fn get_initial_state(
        &self,
        start_node_idx: Option<usize>,
        labels: &[(usize, usize)],
    ) -> DVector<Complex<f64>> {
        let mut res = DVector::from_element(self.matrix.ncols(), Complex::new(0.0, 0.0));
        if res.is_empty() {
            return res;
        }

        let start_node_idx = start_node_idx.unwrap_or(0);
        let adjacent_nodes = labels
            .iter()
            .enumerate()
            .filter(|(_, (x, _))| *x == start_node_idx)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let n = adjacent_nodes.len() as f64;
        let x = Complex::new((1.0 / n).sqrt(), 0.0);
        for &i in adjacent_nodes.iter() {
            res[i] = x;
        }

        res
    }

    pub fn apply(&self, state: DVector<Complex<f64>>) -> DVector<Complex<f64>> {
        &self.matrix * state
    }

    pub fn normalize_unitary(&mut self) -> &TransitionMatrixCorrectionType {
        let n = self.matrix.nrows();
        if n == 0 {
            self.last_normalization_correction = TransitionMatrixCorrectionType::None;
            return &self.last_normalization_correction;
        }

        // correct values and store the amount of correction
        let mut svd = self.matrix.clone().svd(true, true);
        let correction_values = svd.singular_values.clone();
        svd.singular_values.iter_mut().for_each(|x| *x = 1.0);

        let (min_correction, max_correction) = correction_values
            .iter()
            .fold((f64::MAX, f64::MIN), |(min, max), x| {
                (min.min(*x), max.max(*x))
            });
        let largest_abs_correction = max_correction.abs().max(min_correction.abs());
        let correction_difference = max_correction - min_correction;
        let require_non_scalar_correction = { correction_difference > self.max_error };

        if require_non_scalar_correction {
            self.matrix = svd.recompose().expect("SVD recomposition failed");
            self.last_normalization_correction =
                TransitionMatrixCorrectionType::NonScalar(correction_values);
        } else if largest_abs_correction > self.max_error {
            self.matrix = svd.recompose().expect("SVD recomposition failed");
            self.last_normalization_correction =
                TransitionMatrixCorrectionType::Scalar(largest_abs_correction);
        } else {
            self.last_normalization_correction = TransitionMatrixCorrectionType::None;
        }

        &self.last_normalization_correction
    }

    fn display_matrix(&self, ui: &mut egui::Ui, labels: &[(usize, usize)]) {
        if labels.len() != self.matrix.nrows() || labels.len() != self.matrix.ncols() {
            panic!(
                "Matrix dimensions do not match labels: labels:{} vs matrix:{}",
                labels.len(),
                self.matrix.nrows()
            );
        }

        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::Grid::new("normalized_transition_matrix_preview")
                .striped(true)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    // column headers
                    ui.label(""); // empty label to pad for row headers
                    for l in labels.iter() {
                        ui.label(egui::RichText::new(format!("{}->{}", l.0, l.1)).strong());
                    }
                    ui.end_row();

                    // row headers and values
                    for (i, l) in labels.iter().enumerate() {
                        ui.label(egui::RichText::new(format!("{}->{}", l.0, l.1)).strong());
                        for j in 0..labels.len() {
                            if self.matrix[(i, j)].l1_norm() < 1e-3 {
                                ui.label("-");
                            } else {
                                ui.label(format!("{:.03}", self.matrix[(i, j)]));
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }

    fn display_vector(
        ui: &mut egui::Ui,
        vector: &DVector<f64>,
        labels: &[(usize, usize)],
        show_headers: Headers,
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
                    if show_headers == Headers::Show {
                        for l in labels.iter() {
                            ui.label(egui::RichText::new(format!("{}->{}", l.0, l.1)).strong());
                        }
                        ui.end_row();
                    }

                    // values
                    for i in 0..labels.len() {
                        if vector[i] == 0.0 {
                            ui.label("-");
                        } else {
                            ui.label(format!("{:.03}", vector[i]));
                        }
                    }
                });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_unitary_normalization() {
        let unitary_matrix = DMatrix::from_row_slice(
            2,
            2,
            &[
                Complex::new(0.0, 1.0),
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 1.0),
            ],
        ) / Complex::from(2.0_f64.sqrt());
        let mut unitary_transition_matrix = ComplexTransitionMatrix::new(unitary_matrix);
        let correction_type = unitary_transition_matrix.normalize_unitary();
        assert_eq!(correction_type, &TransitionMatrixCorrectionType::None);

        let scalar_unitary_matrix = DMatrix::from_row_slice(
            2,
            2,
            &[
                Complex::new(0.0, 1.0),
                Complex::new(1.0, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 1.0),
            ],
        );
        let mut scalar_transition_matrix = ComplexTransitionMatrix::new(scalar_unitary_matrix);
        let correction_type = scalar_transition_matrix.normalize_unitary();
        assert_eq!(
            correction_type,
            &TransitionMatrixCorrectionType::Scalar(2.0_f64.sqrt() - 1.0)
        );

        let non_scalar_unitary_matrix = DMatrix::from_row_slice(
            2,
            2,
            &[
                Complex::new(0.0, 1.0 + 1e-10),
                Complex::new(1.0 - 1e-10, 0.0),
                Complex::new(1.0, 0.0),
                Complex::new(0.0, 1.1),
            ],
        );
        let mut non_scalar_transition_matrix =
            ComplexTransitionMatrix::new(non_scalar_unitary_matrix);
        let correction_type = non_scalar_transition_matrix.normalize_unitary();
        let correction_values = match correction_type {
            TransitionMatrixCorrectionType::NonScalar(x) => x,
            _ => panic!("Expected non scalar correction"),
        };

        // assert equal with reasonable error (succeeds at 1e-10, fails at 1e-11)
        assert_abs_diff_eq!(
            correction_values.as_slice(),
            &[-0.5_f64, -0.4_f64].as_slice(),
            epsilon = 1e-10
        );
    }
}
