use std::collections::HashSet;

use evalexpr::{context_map, eval_with_context, HashMapContext, Value};
use nalgebra::{Complex, DMatrix};

use super::Editor;

#[derive(Debug, Clone)]
pub struct ComplexMatrixEditor {
    pub matrix: DMatrix<Complex<f64>>,

    math_constants: HashMapContext,

    previous_text_fields: Vec<(String, String)>,
    pub text_fields: Vec<(String, String)>,

    text_fields_modified: bool,

    is_canvas_update_ready: bool,
}

impl Editor for ComplexMatrixEditor {
    fn is_canvas_update_ready(&self) -> bool {
        self.is_canvas_update_ready
    }

    fn on_canvas_updated(&mut self) {
        self.is_canvas_update_ready = false;
    }
}

impl ComplexMatrixEditor {
    pub fn new(size: usize) -> Self {
        let n = size * size;
        let text_fields = vec![(format!("{}", 0.0), format!("{}", 0.0)); n * n];
        Self {
            matrix: DMatrix::from_element(n, n, Complex::new(0.0, 0.0)),
            math_constants: Self::get_math_constants(),
            previous_text_fields: text_fields.clone(),
            text_fields,
            text_fields_modified: false,
            is_canvas_update_ready: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::Grid::new("matrix_editor_grid")
                .striped(true)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    self.show_text_fields(ui);
                    if self.text_fields_modified {
                        self.apply_text_fields();
                    }
                });
        });
    }

    fn show_text_fields(&mut self, ui: &mut egui::Ui) {
        if self.matrix.ncols() == 0 {
            return;
        }

        let n = (self.matrix.nrows() as f64).sqrt() as usize;
        ui.label("");
        for i in 0..n {
            for j in 0..n {
                ui.label(format!("{}->{}", i, j));
                ui.label("");
            }
        }
        ui.end_row();

        let (mut n1, mut n2) = (0, 0);
        for i in 0..self.text_fields.len() {
            if i % self.matrix.ncols() == 0 {
                ui.label(format!("{}->{}", n1, n2));

                n2 += 1;
                if n2 == n {
                    n1 += 1;
                    n2 = 0;
                }
            }

            let re = ui.text_edit_singleline(&mut self.text_fields[i].0);
            if re.lost_focus() {
                self.text_fields_modified = true;
            }

            let im = ui.text_edit_singleline(&mut self.text_fields[i].1);
            if im.lost_focus() {
                self.text_fields_modified = true;
            }

            if (i + 1) % self.matrix.ncols() == 0 {
                ui.end_row();
            }
        }
    }

    fn apply_text_fields(&mut self) {
        for i in 0..self.text_fields.len() {
            let re = eval_with_context(&self.text_fields[i].0, &self.math_constants);
            let re = match re {
                Ok(Value::Int(num)) => num as f64,
                Ok(Value::Float(num)) => num,
                _ => {
                    self.text_fields[i] = self.previous_text_fields[i].clone();
                    continue;
                }
            };

            let im = eval_with_context(&self.text_fields[i].1, &self.math_constants);
            let im = match im {
                Ok(Value::Int(num)) => num as f64,
                Ok(Value::Float(num)) => num,
                _ => {
                    self.text_fields[i] = self.previous_text_fields[i].clone();
                    continue;
                }
            };

            self.set_ith_element(i, Complex::new(re, im));
        }
        self.previous_text_fields = self.text_fields.clone();
        self.text_fields_modified = false;
        self.is_canvas_update_ready = true;
    }

    fn set_ith_element(&mut self, i: usize, value: Complex<f64>) {
        let (row, col) = self.ith_index_to_row_col(i);
        self.matrix[(row, col)] = value;
    }

    fn ith_index_to_row_col(&self, i: usize) -> (usize, usize) {
        let nrows = self.matrix.nrows();
        let ncols = self.matrix.ncols();
        (i / nrows, i % ncols)
    }

    fn get_math_constants() -> HashMapContext {
        context_map! {
            "pi" => std::f64::consts::PI,
            "e" => std::f64::consts::E,
            "tau" => std::f64::consts::TAU,
        }
        .unwrap()
    }

    pub(crate) fn resize_matrix(&mut self, size: usize) {
        if self.matrix.nrows() == size {
            return;
        }

        let old_matrix = self.matrix.clone();
        let old_size = old_matrix.nrows();
        self.matrix = DMatrix::from_element(size, size, Complex::new(0.0, 0.0));
        for i in 0..size {
            for j in 0..size {
                if i < old_size && j < old_size {
                    self.matrix[(i, j)] = old_matrix[(i, j)];
                }
            }
        }

        let old_text_fields = self.text_fields.clone();
        self.text_fields = vec![(format!("{}", 0.0), format!("{}", 0.0)); size * size];
        for i in 0..size.min(old_size) {
            for j in 0..size.min(old_size) {
                if i < old_size && j < old_size {
                    self.text_fields[i * size + j] = old_text_fields[i * old_size + j].clone();
                }
            }
        }
        self.previous_text_fields = self.text_fields.clone();
    }

    // BUG this doesn't work with the double-edge adjacency matrix
    pub(crate) fn remove_node(&mut self, node_idxs: Vec<usize>) {
        let n = node_idxs.len();
        let mut new_matrix = DMatrix::from_element(
            self.matrix.nrows() - n,
            self.matrix.ncols() - n,
            Complex::new(0.0, 0.0),
        );
        let mut new_text_fields = vec![
            (format!("{}", 0.0), format!("{}", 0.0));
            (self.matrix.nrows() - n) * (self.matrix.ncols() - n)
        ];
        let mut row_idx = 0;
        for i in 0..self.matrix.nrows() {
            if node_idxs.contains(&i) {
                continue;
            }
            let mut col_idx = 0;
            for j in 0..self.matrix.ncols() {
                if node_idxs.contains(&j) {
                    continue;
                }

                new_matrix[(row_idx, col_idx)] = self.matrix[(i, j)];
                new_text_fields[row_idx * (self.matrix.nrows() - n) + col_idx] =
                    self.text_fields[i * self.matrix.nrows() + j].clone();

                col_idx += 1;
            }
            row_idx += 1;
        }
        self.matrix = new_matrix;
        self.text_fields = new_text_fields;
        self.previous_text_fields = self.text_fields.clone();
    }

    /// To make it more clear to the user what values should be adjusted, the
    /// matrix is updated using the lines on the canvas. All the edges are
    /// considered to be undirected. Anywhere a line exists between a pair of
    /// nodes (i,j) the matrix will show a complex number 1+1i.
    pub(crate) fn update_from_canvas_edges(&mut self, edges: &Vec<(usize, usize)>) {
        let matrix = &mut self.matrix;

        let edges: HashSet<(usize, usize)> = HashSet::from_iter(edges.iter().cloned());
        for i in 0..matrix.nrows() {
            for j in 0..matrix.ncols() {
                let matrix_edge_exists = matrix[(i, j)] != Complex::new(0.0, 0.0)
                    || matrix[(j, i)] != Complex::new(0.0, 0.0);
                let canvas_edge_exists = edges.contains(&(i, j)) || edges.contains(&(j, i));

                if matrix_edge_exists && !canvas_edge_exists {
                    matrix[(i, j)] = Complex::new(0.0, 0.0);
                    matrix[(j, i)] = Complex::new(0.0, 0.0);
                } else if !matrix_edge_exists && canvas_edge_exists {
                    matrix[(i, j)] = Complex::new(1.0, 1.0);
                    matrix[(j, i)] = Complex::new(1.0, 1.0);
                }
            }
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SerializedMatrixEditor {
    size: usize,
    matrix: Vec<(f64, f64)>,
    text_fields: Vec<(String, String)>,
}
impl From<ComplexMatrixEditor> for SerializedMatrixEditor {
    fn from(m: ComplexMatrixEditor) -> Self {
        Self {
            size: m.matrix.nrows(),
            matrix: m.matrix.iter().map(|x| (x.re, x.im)).collect(),
            text_fields: m.text_fields,
        }
    }
}
impl From<SerializedMatrixEditor> for ComplexMatrixEditor {
    fn from(m: SerializedMatrixEditor) -> Self {
        Self {
            matrix: DMatrix::from_vec(
                m.size,
                m.size,
                m.matrix
                    .iter()
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect(),
            ),
            math_constants: Self::get_math_constants(),
            previous_text_fields: m.text_fields.clone(),
            text_fields: m.text_fields,
            text_fields_modified: false,
            is_canvas_update_ready: false,
        }
    }
}
impl serde::Serialize for ComplexMatrixEditor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        SerializedMatrixEditor::from(self.clone()).serialize(serializer)
    }
}
impl<'de> serde::Deserialize<'de> for ComplexMatrixEditor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        SerializedMatrixEditor::deserialize(deserializer).map(Self::from)
    }
}
