use anyhow::{anyhow, Error, Result};
use nalgebra::{Complex, DMatrix, DVector, Dyn};

use crate::transition_matrix_correction_type::TransitionMatrixCorrectionType;

#[derive(Debug, Clone)]
pub struct ComplexTransitionMatrix {
    pub matrix: DMatrix<Complex<f64>>,
    max_error: f64,
}

impl std::fmt::Display for ComplexTransitionMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl TryFrom<&DMatrix<Complex<f64>>> for ComplexTransitionMatrix {
    type Error = Error;

    fn try_from(stochastic_matrix: &DMatrix<Complex<f64>>) -> Result<Self, Self::Error> {
        if !stochastic_matrix.is_square() {
            return Err(anyhow!("Matrix is not square"));
        }

        let n = stochastic_matrix.nrows();
        let m = n.pow(2);
        let mut matrix = DMatrix::from_element(m, m, Complex::new(0.0, 0.0));
        for col_offset in 0..n {
            for j in 0..n {
                for i in 0..n {
                    let row = i + (j % n) * n;
                    let col = j + col_offset * n;
                    matrix[(row, col)] = stochastic_matrix[(i, j)];
                }
            }
        }
        let mut res = Self::new(matrix)?;
        res.normalize_unitary();
        Ok(res)
    }
}

impl ComplexTransitionMatrix {
    pub fn new(matrix: DMatrix<Complex<f64>>) -> Result<Self> {
        if matrix.nrows() != matrix.ncols() {
            return Err(anyhow!("Matrix must be square"));
        }
        Ok(Self {
            matrix,
            max_error: 1e-10,
        })
    }

    pub fn get_initial_state(&self, start_node_idx: &Option<usize>) -> DVector<Complex<f64>> {
        let nnodes = (self.matrix.ncols() as f64).sqrt() as usize;
        let mut res = DVector::from_element(self.matrix.ncols(), Complex::new(0.0, 0.0));
        if res.len() == 0 {
            return res;
        }
        let start_node_idx = start_node_idx
            .and_then(|x| Some(x * nnodes + x))
            .unwrap_or(0);
        res[start_node_idx] = Complex::new(1.0, 0.0);
        res
    }

    pub fn apply(&self, state: DVector<Complex<f64>>) -> Result<DVector<Complex<f64>>> {
        if state.len() != self.matrix.ncols() {
            return Err(anyhow!("Matrix dimensions do not match"));
        }
        Ok(&self.matrix * state)
    }

    pub fn normalize_unitary(&mut self) -> TransitionMatrixCorrectionType {
        let n = self.matrix.nrows();
        let mut correction_vals = DVector::from_element(n, 0.0);
        let mut svd = self.matrix.clone().svd(true, true);
        svd.singular_values
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| {
                correction_vals[i] = 1.0 - *x;
                *x = 1.0;
            });

        let (min_correction, max_correction) = correction_vals
            .iter()
            .fold((f64::MAX, f64::MIN), |(min, max), x| {
                (min.min(*x), max.max(*x))
            });
        let largest_abs_correction = max_correction.abs().max(min_correction.abs());
        let correction_vals_difference = max_correction - min_correction;
        let require_non_scalar_correction = { correction_vals_difference > self.max_error };

        if require_non_scalar_correction {
            Self::make_svd_unitary(&mut svd);
            self.matrix = svd.recompose().expect("SVD recomposition failed");
            TransitionMatrixCorrectionType::NonScalar(correction_vals)
        } else if largest_abs_correction > self.max_error {
            Self::make_svd_unitary(&mut svd);
            self.matrix = svd.recompose().expect("SVD recomposition failed");
            TransitionMatrixCorrectionType::Scalar(largest_abs_correction)
        } else {
            TransitionMatrixCorrectionType::None
        }
    }

    fn make_svd_unitary(svd: &mut nalgebra::SVD<Complex<f64>, Dyn, Dyn>) {
        svd.singular_values.iter_mut().for_each(|x| {
            *x = 1.0;
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
        let mut unitary_transition_matrix = ComplexTransitionMatrix::new(unitary_matrix).unwrap();
        let correction_type = unitary_transition_matrix.normalize_unitary();
        assert_eq!(correction_type, TransitionMatrixCorrectionType::None);

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
        let mut scalar_transition_matrix =
            ComplexTransitionMatrix::new(scalar_unitary_matrix).unwrap();
        let correction_type = scalar_transition_matrix.normalize_unitary();
        assert_eq!(
            correction_type,
            TransitionMatrixCorrectionType::Scalar(2.0_f64.sqrt() - 1.0)
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
            ComplexTransitionMatrix::new(non_scalar_unitary_matrix).unwrap();
        let correction_type = non_scalar_transition_matrix.normalize_unitary();
        let correction_vals = match correction_type {
            TransitionMatrixCorrectionType::NonScalar(x) => x,
            _ => panic!("Expected non scalar correction"),
        };

        // assert equal with reasonable error (succeeds at 1e-10, fails at 1e-11)
        assert_abs_diff_eq!(
            correction_vals.as_slice(),
            &[-0.5_f64, -0.4_f64].as_slice(),
            epsilon = 1e-10
        );
    }
}
