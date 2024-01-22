use evalexpr::{context_map, eval_with_context, HashMapContext, Value};
use nalgebra::DMatrix;

#[derive(Debug, Clone)]
pub struct MatrixEditor {
    pub matrix: DMatrix<f64>,

    math_constants: HashMapContext,

    previous_text_fields: Vec<String>,
    pub text_fields: Vec<String>,

    text_fields_modified: bool,
}

impl MatrixEditor {
    pub fn new(size: usize) -> Self {
        let text_fields = vec![format!("{}", 0.0); size * size];
        Self {
            matrix: DMatrix::from_element(size, size, 0.0),
            math_constants: Self::get_math_constants(),
            previous_text_fields: text_fields.clone(),
            text_fields,
            text_fields_modified: false,
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
        for i in 0..self.text_fields.len() {
            let res = ui.text_edit_singleline(&mut self.text_fields[i]);
            if res.lost_focus() {
                self.text_fields_modified = true;
            }

            if (i + 1) % self.matrix.ncols() == 0 {
                ui.end_row();
            }
        }
    }

    fn apply_text_fields(&mut self) {
        for i in 0..self.text_fields.len() {
            let res = eval_with_context(&self.text_fields[i], &self.math_constants);
            match res {
                Ok(Value::Int(num)) => self.set_ith_element(i, num as f64),
                Ok(Value::Float(num)) => self.set_ith_element(i, num),
                _ => {
                    self.text_fields[i] = self.previous_text_fields[i].clone();
                    continue;
                }
            };
        }
        self.previous_text_fields = self.text_fields.clone();
        self.text_fields_modified = false;
    }

    fn set_ith_element(&mut self, i: usize, value: f64) {
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
        self.matrix = DMatrix::from_element(size, size, 0.0);
        for i in 0..size {
            for j in 0..size {
                if i < old_size && j < old_size {
                    self.matrix[(i, j)] = old_matrix[(i, j)];
                }
            }
        }

        let old_text_fields = self.text_fields.clone();
        self.text_fields = vec![format!("{}", 0.0); size * size];
        for i in 0..size.min(old_size) {
            for j in 0..size.min(old_size) {
                if i < old_size && j < old_size {
                    self.text_fields[i * size + j] = old_text_fields[i * old_size + j].clone();
                }
            }
        }
        self.previous_text_fields = self.text_fields.clone();
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SerializedMatrixEditor {
    size: usize,
    matrix: Vec<f64>,
    text_fields: Vec<String>,
}
impl From<MatrixEditor> for SerializedMatrixEditor {
    fn from(m: MatrixEditor) -> Self {
        Self {
            size: m.matrix.nrows(),
            matrix: m.matrix.as_slice().to_vec(),
            text_fields: m.text_fields,
        }
    }
}
impl From<SerializedMatrixEditor> for MatrixEditor {
    fn from(m: SerializedMatrixEditor) -> Self {
        Self {
            matrix: DMatrix::from_vec(m.size, m.size, m.matrix),
            math_constants: Self::get_math_constants(),
            previous_text_fields: m.text_fields.clone(),
            text_fields: m.text_fields,
            text_fields_modified: false,
        }
    }
}
impl serde::Serialize for MatrixEditor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        SerializedMatrixEditor::from(self.clone()).serialize(serializer)
    }
}
impl<'de> serde::Deserialize<'de> for MatrixEditor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        SerializedMatrixEditor::deserialize(deserializer).map(Self::from)
    }
}