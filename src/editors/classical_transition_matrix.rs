use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nalgebra::{DMatrix, DVector};

use super::TransitionMatrixCorrectionType;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassicalTransitionMatrix {
    pub matrix: DMatrix<f64>,

    row_idx_map: HashMap<(usize, usize), usize>,
    col_idx_map: HashMap<(usize, usize), usize>,
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

        let col_idx_map = (0..n)
            .flat_map(|x| (0..n).map(move |y| (x, y)))
            .enumerate()
            .map(|(idx, (i, j))| ((i, j), idx))
            .collect::<HashMap<(usize, usize), usize>>();

        let row_idx_map = (0..n)
            .flat_map(|x| (0..n).map(move |y| (x, y)))
            .enumerate()
            .filter(|(i, _)| matrix.row(*i).iter().any(|y| *y != 0.0))
            .enumerate()
            .map(|(idx, (_, (i, j)))| ((i, j), idx))
            .collect::<HashMap<(usize, usize), usize>>();

        let n_nonzero_rows = row_idx_map.len();
        let mut res_matrix = DMatrix::from_element(n_nonzero_rows, m, 0.0);
        for (i, row) in (0..m)
            .filter(|x| matrix.row(*x).iter().any(|y| *y != 0.0))
            .enumerate()
        {
            for col in 0..m {
                res_matrix[(i, col)] = matrix[(row, col)];
            }
        }

        let mut res = Self {
            matrix: res_matrix,
            row_idx_map,
            col_idx_map,
        };
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
        if self.matrix.ncols() != state.len() {
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
        let target_matrix = DMatrix::from_fn(6, 9, |i, j| match (i, j) {
            (0, 0) => 0.3,
            (0, 3) => 0.3,
            (0, 6) => 0.3,
            (1, 0) => 0.5,
            (1, 3) => 0.5,
            (1, 6) => 0.5,
            (2, 0) => 0.2,
            (2, 3) => 0.2,
            (2, 6) => 0.2,
            (3, 1) => 1.0,
            (3, 4) => 1.0,
            (3, 7) => 1.0,
            (4, 2) => 0.7,
            (4, 5) => 0.7,
            (4, 8) => 0.7,
            (5, 2) => 0.3,
            (5, 5) => 0.3,
            (5, 8) => 0.3,
            _ => 0.0,
        });
        println!("OUTPUT MATRIX: {}", output_matrix.matrix);
        println!("TARGET MATRIX: {}", target_matrix);
        assert_eq!(output_matrix.matrix, target_matrix);

        let target_col_index_map = HashMap::from([
            ((0, 0), 0),
            ((0, 1), 1),
            ((0, 2), 2),
            ((1, 0), 3),
            ((1, 1), 4),
            ((1, 2), 5),
            ((2, 0), 6),
            ((2, 1), 7),
            ((2, 2), 8),
        ]);
        println!("OUTPUT COL MAP: {:?}", output_matrix.col_idx_map);
        println!("TARGET COL MAP: {:?}", target_col_index_map);
        assert_eq!(output_matrix.col_idx_map, target_col_index_map);

        let target_row_index_map = HashMap::from([
            ((0, 0), 0),
            ((0, 1), 1),
            ((0, 2), 2),
            ((1, 2), 3),
            ((2, 0), 4),
            ((2, 1), 5),
        ]);
        println!("OUTPUT ROW MAP: {:?}", output_matrix.row_idx_map);
        println!("TARGET ROW MAP: {:?}", target_row_index_map);
        assert_eq!(output_matrix.row_idx_map, target_row_index_map);
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
        let target_matrix = DMatrix::from_fn(9, 36, |i, j| match (i, j) {
            (0, 0) => 0.5,
            (0, 12) => 0.5,
            (0, 18) => 0.5,
            (0, 24) => 0.5,
            (0, 30) => 0.5,
            (0, 6) => 0.5,
            (1, 0) => 0.5,
            (1, 12) => 0.5,
            (1, 18) => 0.5,
            (1, 24) => 0.5,
            (1, 30) => 0.5,
            (1, 6) => 0.5,
            (2, 1) => 0.5,
            (2, 13) => 0.5,
            (2, 19) => 0.5,
            (2, 25) => 0.5,
            (2, 31) => 0.5,
            (2, 7) => 0.5,
            (3, 1) => 0.5,
            (3, 13) => 0.5,
            (3, 19) => 0.5,
            (3, 25) => 0.5,
            (3, 31) => 0.5,
            (3, 7) => 0.5,
            (4, 14) => 0.5,
            (4, 2) => 0.5,
            (4, 20) => 0.5,
            (4, 26) => 0.5,
            (4, 32) => 0.5,
            (4, 8) => 0.5,
            (5, 14) => 0.5,
            (5, 2) => 0.5,
            (5, 20) => 0.5,
            (5, 26) => 0.5,
            (5, 32) => 0.5,
            (5, 8) => 0.5,
            (6, 15) => 1.0,
            (6, 21) => 1.0,
            (6, 27) => 1.0,
            (6, 3) => 1.0,
            (6, 33) => 1.0,
            (6, 9) => 1.0,
            (7, 10) => 1.0,
            (7, 16) => 1.0,
            (7, 22) => 1.0,
            (7, 28) => 1.0,
            (7, 34) => 1.0,
            (7, 4) => 1.0,
            (8, 11) => 1.0,
            (8, 17) => 1.0,
            (8, 23) => 1.0,
            (8, 29) => 1.0,
            (8, 35) => 1.0,
            (8, 5) => 1.0,
            _ => 0.0,
        });
        println!("OUTPUT MATRIX: {}", output_matrix.matrix);
        println!("TARGET MATRIX: {}", target_matrix);
        assert_eq!(output_matrix.matrix, target_matrix);

        let target_col_index_map = HashMap::from([
            ((0, 0), 0),
            ((0, 1), 1),
            ((0, 2), 2),
            ((0, 3), 3),
            ((0, 4), 4),
            ((0, 5), 5),
            ((1, 0), 6),
            ((1, 1), 7),
            ((1, 2), 8),
            ((1, 3), 9),
            ((1, 4), 10),
            ((1, 5), 11),
            ((2, 0), 12),
            ((2, 1), 13),
            ((2, 2), 14),
            ((2, 3), 15),
            ((2, 4), 16),
            ((2, 5), 17),
            ((3, 0), 18),
            ((3, 1), 19),
            ((3, 2), 20),
            ((3, 3), 21),
            ((3, 4), 22),
            ((3, 5), 23),
            ((4, 0), 24),
            ((4, 1), 25),
            ((4, 2), 26),
            ((4, 3), 27),
            ((4, 4), 28),
            ((4, 5), 29),
            ((5, 0), 30),
            ((5, 1), 31),
            ((5, 2), 32),
            ((5, 3), 33),
            ((5, 4), 34),
            ((5, 5), 35),
        ]);
        println!("OUTPUT COL MAP: {:?}", output_matrix.col_idx_map);
        println!("TARGET COL MAP: {:?}", target_col_index_map);
        assert_eq!(output_matrix.col_idx_map, target_col_index_map);

        let target_row_index_map = HashMap::from([
            ((4, 4), 7),
            ((5, 5), 8),
            ((0, 2), 1),
            ((2, 5), 5),
            ((3, 3), 6),
            ((0, 1), 0),
            ((2, 4), 4),
            ((1, 3), 2),
            ((1, 4), 3),
        ]);
        println!("OUTPUT ROW MAP: {:?}", output_matrix.row_idx_map);
        println!("TARGET ROW MAP: {:?}", target_row_index_map);
        assert_eq!(output_matrix.row_idx_map, target_row_index_map);
    }
}
