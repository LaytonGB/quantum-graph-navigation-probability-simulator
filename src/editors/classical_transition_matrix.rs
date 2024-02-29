use anyhow::{anyhow, Result};
use nalgebra::{DMatrix, DVector};

use super::TransitionMatrixCorrectionType;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassicalTransitionMatrix {
    pub matrix: DMatrix<f64>,
}

impl std::fmt::Display for ClassicalTransitionMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl TryFrom<&DMatrix<f64>> for ClassicalTransitionMatrix {
    type Error = anyhow::Error;

    fn try_from(stochastic_matrix: &DMatrix<f64>) -> Result<Self, Self::Error> {
        if !stochastic_matrix.is_square() {
            return Err(anyhow!("Matrix is not square"));
        }

        let n = stochastic_matrix.nrows();
        let m = n.pow(2);
        let mut matrix = DMatrix::from_element(m, m, 0.0);

        for col_offset in 0..n {
            for j in 0..n {
                for i in 0..n {
                    let row = i + (j % n) * n;
                    let col = j + col_offset * n;
                    matrix[(row, col)] = stochastic_matrix[(i, j)];
                }
            }
        }

        let mut res = Self { matrix };
        res.normalize_stochastic();

        if res.matrix.iter().any(|x| x.is_nan()) {
            Err(anyhow!("Matrix is not stochastic"))
        } else {
            Ok(res)
        }
    }
}

impl ClassicalTransitionMatrix {
    pub fn get_initial_state(&self, start_node_idx: &Option<usize>) -> DVector<f64> {
        let nnodes = (self.matrix.ncols() as f64).sqrt() as usize;
        let mut res = DVector::from_element(self.matrix.ncols(), 0.0);
        if res.len() == 0 {
            return res;
        }
        let start_node_idx = start_node_idx
            .and_then(|x| Some(x * nnodes + x))
            .unwrap_or(0);
        res[start_node_idx] = 1.0;
        res
    }

    pub fn apply(&self, state: DVector<f64>) -> Result<DVector<f64>> {
        if state.len() != self.matrix.ncols() {
            return Err(anyhow::anyhow!("Matrix dimensions do not match"));
        }
        Ok(&self.matrix * state)
    }

    fn normalize_stochastic(&mut self) -> TransitionMatrixCorrectionType {
        let n = self.matrix.nrows();

        let mut res = TransitionMatrixCorrectionType::None;
        for j in 0..n {
            let sum = self.matrix.column(j).iter().sum::<f64>();
            if sum != 0.0 && sum != 1.0 {
                match res {
                    TransitionMatrixCorrectionType::None => {
                        res = TransitionMatrixCorrectionType::Scalar(1.0 / sum)
                    }
                    TransitionMatrixCorrectionType::Scalar(prev_res) => {
                        res = TransitionMatrixCorrectionType::NonScalar(DVector::from_fn(
                            n,
                            |row, col| match row {
                                _ if col == j - 1 => prev_res,
                                _ if col == j => 1.0 / sum,
                                _ => 0.0,
                            },
                        ));
                    }
                    TransitionMatrixCorrectionType::NonScalar(ref mut res) => res[j] = 1.0 / sum,
                }

                for i in 0..n {
                    self.matrix[(i, j)] /= sum;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_stochastic_matrix_3nodes() {
        let input_matrix = DMatrix::from_fn(3, 3, |i, j| match (i, j) {
            (0, 0) => 0.3,
            (0, 2) => 0.7,
            (1, 0) => 0.5,
            (1, 2) => 0.3,
            (2, 0) => 0.2,
            (2, 1) => 1.0,
            _ => 0.0,
        });
        let output_matrix = ClassicalTransitionMatrix::try_from(&input_matrix).unwrap();
        let target_matrix = DMatrix::from_fn(9, 9, |i, j| match (i, j) {
            (0, 0) => 0.3,
            (0, 3) => 0.3,
            (0, 6) => 0.3,
            (1, 0) => 0.5,
            (1, 3) => 0.5,
            (1, 6) => 0.5,
            (2, 0) => 0.2,
            (2, 3) => 0.2,
            (2, 6) => 0.2,
            (5, 1) => 1.0,
            (5, 4) => 1.0,
            (5, 7) => 1.0,
            (6, 2) => 0.7,
            (6, 5) => 0.7,
            (6, 8) => 0.7,
            (7, 2) => 0.3,
            (7, 5) => 0.3,
            (7, 8) => 0.3,
            _ => 0.0,
        });
        assert_eq!(output_matrix.matrix, target_matrix);
    }

    #[test]
    fn test_from_stochastic_matrix_6nodes() {
        let input_matrix = DMatrix::from_fn(6, 6, |i, j| match (i, j) {
            (1, 0) => 0.5,
            (2, 0) => 0.5,
            (3, 1) => 0.5,
            (3, 3) => 1.0,
            (4, 1) => 0.5,
            (4, 2) => 0.5,
            (4, 4) => 1.0,
            (5, 2) => 0.5,
            (5, 5) => 1.0,
            _ => 0.0,
        });
        let output_matrix = ClassicalTransitionMatrix::try_from(&input_matrix).unwrap();
        let target_matrix = DMatrix::from_fn(36, 36, |i, j| match (i, j) {
            (1, 0) => 0.5,
            (1, 12) => 0.5,
            (1, 18) => 0.5,
            (1, 24) => 0.5,
            (1, 30) => 0.5,
            (1, 6) => 0.5,
            (10, 1) => 0.5,
            (10, 13) => 0.5,
            (10, 19) => 0.5,
            (10, 25) => 0.5,
            (10, 31) => 0.5,
            (10, 7) => 0.5,
            (16, 14) => 0.5,
            (16, 2) => 0.5,
            (16, 20) => 0.5,
            (16, 26) => 0.5,
            (16, 32) => 0.5,
            (16, 8) => 0.5,
            (17, 14) => 0.5,
            (17, 2) => 0.5,
            (17, 20) => 0.5,
            (17, 26) => 0.5,
            (17, 32) => 0.5,
            (17, 8) => 0.5,
            (2, 0) => 0.5,
            (2, 12) => 0.5,
            (2, 18) => 0.5,
            (2, 24) => 0.5,
            (2, 30) => 0.5,
            (2, 6) => 0.5,
            (21, 15) => 1.0,
            (21, 21) => 1.0,
            (21, 27) => 1.0,
            (21, 3) => 1.0,
            (21, 33) => 1.0,
            (21, 9) => 1.0,
            (28, 10) => 1.0,
            (28, 16) => 1.0,
            (28, 22) => 1.0,
            (28, 28) => 1.0,
            (28, 34) => 1.0,
            (28, 4) => 1.0,
            (35, 11) => 1.0,
            (35, 17) => 1.0,
            (35, 23) => 1.0,
            (35, 29) => 1.0,
            (35, 35) => 1.0,
            (35, 5) => 1.0,
            (9, 1) => 0.5,
            (9, 13) => 0.5,
            (9, 19) => 0.5,
            (9, 25) => 0.5,
            (9, 31) => 0.5,
            (9, 7) => 0.5,
            _ => 0.0,
        });
        assert_eq!(output_matrix.matrix, target_matrix);
    }
}
