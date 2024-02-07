#[derive(Debug)]
pub struct TransitionMatrix {
    pub matrix: DMatrix<f64>,
    idx_map: HashMap<(usize, usize), usize>,
}

impl TransitionMatrix {
    /// WARNING: only works for square matrices
    pub fn new(edge_count: usize, stochastic_matrix: DMatrix<f64>) -> Self {
        let n = stochastic_matrix.ncols();
        let matrix = DMatrix::from_element(edge_count, edge_count, 0.0);
        for i in 0..n {
            for j in 0..n {
                if stochastic_matrix[(i, j)] > 0.0 {}
            }
        }
        let idx_map = HashMap::new();
        Self { matrix, idx_map }
    }
}
