use anyhow::Result;
use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationMode {
    Stochastic,
    Unitary,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionMatrix {
    pub matrix: DMatrix<f64>,
}

impl std::fmt::Display for TransitionMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl TryFrom<&DMatrix<f64>> for TransitionMatrix {
    type Error = &'static str;

    fn try_from(stochastic_matrix: &DMatrix<f64>) -> Result<Self, Self::Error> {
        if !stochastic_matrix.is_square() {
            return Err("Matrix is not square");
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
        res.normalize_self(NormalizationMode::Stochastic);
        Ok(res)
    }
}

impl TransitionMatrix {
    pub fn get_initial_state(&self) -> DVector<f64> {
        let mut res = DVector::from_element(self.matrix.ncols(), 0.0);
        if res.len() == 0 {
            return res;
        }
        res[0] = 1.0;
        res
    }

    pub fn apply(&self, state: DVector<f64>) -> Result<DVector<f64>> {
        if state.len() != self.matrix.ncols() {
            return Err(anyhow::anyhow!("Matrix dimensions do not match"));
        }
        Ok(&self.matrix * state)
    }

    fn normalize_self(&mut self, normalization_mode: NormalizationMode) {
        match normalization_mode {
            NormalizationMode::Stochastic => self.normalize_stochastic(),
            NormalizationMode::Unitary => self.normalize_unitary(),
        }
    }

    fn normalize_stochastic(&mut self) {
        let n = self.matrix.nrows();
        for j in 0..n {
            let sum = self.matrix.column(j).iter().sum::<f64>();
            if sum > 0.0 {
                for i in 0..n {
                    self.matrix[(i, j)] /= sum;
                }
            }
        }
    }

    fn normalize_unitary(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_stochastic_matrix() {
        let input_matrix = DMatrix::from_fn(3, 3, |i, j| match (i, j) {
            (0, 0) => 0.3,
            (0, 2) => 0.7,
            (1, 0) => 0.5,
            (1, 2) => 0.3,
            (2, 0) => 0.2,
            (2, 1) => 1.0,
            _ => 0.0,
        });
        let output_matrix = TransitionMatrix::try_from(&input_matrix).unwrap();
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
}
