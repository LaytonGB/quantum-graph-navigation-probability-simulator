use std::collections::{HashMap, HashSet};

use evalexpr::{context_map, eval_with_context, HashMapContext, Value};
use nalgebra::{Complex, DMatrix};

use super::Editor;

#[derive(Debug, Clone)]
pub struct ComplexMatrixEditor {
    pub scatter_matrix: DMatrix<Complex<f64>>,
    pub propagation_matrix: DMatrix<Complex<f64>>,
    pub combined_matrix: DMatrix<Complex<f64>>,

    math_constants: HashMapContext,

    adjacency_list: HashMap<usize, Vec<usize>>,
    index_map: HashMap<(usize, usize), usize>,
    labels: Vec<(usize, usize)>,

    previous_text_fields: Vec<Vec<Vec<(String, String)>>>,
    pub text_fields: Vec<Vec<Vec<(String, String)>>>,

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
    pub fn new(edges: &Vec<(usize, usize)>) -> Self {
        // let node_edge_counts = edges.iter().fold(HashMap::new(), |mut m, (i, j)| {
        //     m.entry(*i).and_modify(|e| *e += 1).or_insert(1);
        //     m.entry(*j).and_modify(|e| *e += 1).or_insert(1);
        //     m
        // });

        let mut adjacency_list = edges.iter().fold(HashMap::new(), |mut m, (i, j)| {
            m.entry(*i)
                .and_modify(|e: &mut Vec<usize>| e.push(*j))
                .or_insert(vec![*j]);
            m.entry(*j)
                .and_modify(|e: &mut Vec<usize>| e.push(*i))
                .or_insert(vec![*i]);
            m
        });
        adjacency_list.values_mut().for_each(|v| v.sort_unstable());

        let mut labels = adjacency_list.iter().collect::<Vec<_>>();
        labels.sort_unstable();
        let labels: Vec<(usize, usize)> = labels
            .into_iter()
            .flat_map(|(i, v)| v.iter().map(move |j| (*i, *j)))
            .collect();

        let half_edge_count = labels.len();

        let index_map = labels
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut m, (i, (a, b))| {
                m.insert((*a, *b), i);
                m
            });

        // TODO allow customization
        // currently always assuming line-graph
        let scatter_matrix =
            DMatrix::from_element(half_edge_count, half_edge_count, Complex::new(0.0, 0.0));

        // FIXME populate the scatter matrix
        // let nodes_with_edges_count = node_edge_counts.len();
        // for i in 0..nodes_with_edges_count {
        //     scatter_matrix[(i, i)] = Complex::new(1.0, 0.0);
        //     scatter_matrix[(i + 1, i)] = Complex::new(0.0, 1.0);
        //     scatter_matrix[(i, i + 1)] = Complex::new(0.0, 1.0);
        //     scatter_matrix[(i + 1, i + 1)] = Complex::new(1.0, 0.0);
        // }

        // TODO allow customization
        // currently always assuming line-graph
        let propagation_matrix = DMatrix::from_fn(half_edge_count, half_edge_count, |i, j| {
            // where the coordinates point to some node that has 2 edges, eg 0->0, 0->1
            // being on some edge 0->1 would then place the particle on edge 1->0
            // 0, 1
            // 1, 0
            if i == j + 1 || j == i + 1 {
                Complex::new(1.0, 0.0)
            } else {
                Complex::new(0.0, 0.0)
            }
        });

        let mut sorted_adjacency_list = adjacency_list.keys().collect::<Vec<_>>();
        sorted_adjacency_list.sort_unstable();
        let text_fields = sorted_adjacency_list.iter().fold(Vec::new(), |mut v, k| {
            let connections = adjacency_list.get(k).unwrap();
            v.push(vec![
                vec![
                    (String::from("0"), String::from("0"));
                    connections.len()
                ];
                connections.len()
            ]);
            v
        });
        Self {
            propagation_matrix,
            scatter_matrix,
            combined_matrix: DMatrix::from_element(
                half_edge_count,
                half_edge_count,
                Complex::new(0.0, 0.0),
            ),

            math_constants: Self::get_math_constants(),

            adjacency_list,
            index_map,
            labels,

            previous_text_fields: text_fields.clone(),
            text_fields,

            text_fields_modified: false,
            is_canvas_update_ready: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            self.show_text_fields(ui);
            if self.text_fields_modified {
                self.apply_text_fields();
            }
        });
    }

    fn show_text_fields(&mut self, ui: &mut egui::Ui) {
        if self.adjacency_list.is_empty() {
            ui.label("No edges to show");
            return;
        }

        // sort nodes into order
        let mut from_nodes = self.adjacency_list.keys().copied().collect::<Vec<_>>();
        from_nodes.sort_unstable();

        // display section for each node's connections
        for (i, from) in from_nodes.iter().enumerate() {
            let connections = self.adjacency_list.get(from).unwrap();
            ui.collapsing(format!("Node {}", from), |ui| {
                egui::Grid::new(format!("node_{}_editor_grid", from))
                    .striped(true)
                    .spacing([10.0, 10.0])
                    .show(ui, |ui| {
                        // add column headers
                        ui.label(""); // empty label to pad for row headers
                        for to in connections.iter() {
                            ui.label(format!("{}->{}", from, to));
                            ui.label(""); // empty label to keep aligned with text fields (real + imaginary)
                        }
                        ui.end_row();

                        for (j, to) in connections.iter().enumerate() {
                            ui.label(format!("{}->{}", from, to)); // row header

                            let text_fields = &mut self.text_fields[i][j];
                            for field in text_fields.iter_mut() {
                                ui.text_edit_singleline(&mut field.0);
                                ui.text_edit_singleline(&mut field.1);
                            }
                            ui.end_row();
                        }
                    });
            });
        }
    }

    fn apply_text_fields(&mut self) {
        // for i in 0..self.text_fields.len() {
        //     for j in 0..self.text_fields[i].len() {
        //         let re = eval_with_context(&self.text_fields[i][j].0, &self.math_constants);
        //         let re = match re {
        //             Ok(Value::Int(num)) => num as f64,
        //             Ok(Value::Float(num)) => num,
        //             _ => {
        //                 self.text_fields[i][j] = self.previous_text_fields[i][j].clone();
        //                 continue;
        //             }
        //         };

        //         let im = eval_with_context(&self.text_fields[i][j].1, &self.math_constants);
        //         let im = match im {
        //             Ok(Value::Int(num)) => num as f64,
        //             Ok(Value::Float(num)) => num,
        //             _ => {
        //                 self.text_fields[i] = self.previous_text_fields[i].clone();
        //                 continue;
        //             }
        //         };

        //         self.set_value(i, j, Complex::new(re, im));
        //     }
        // }
        // self.previous_text_fields = self.text_fields.clone();
        // self.text_fields_modified = false;
        // self.is_canvas_update_ready = true;
    }

    fn set_value(&mut self, row: usize, col: usize, value: Complex<f64>) {
        self.combined_matrix[(row, col)] = value;
    }

    fn get_math_constants() -> HashMapContext {
        context_map! {
            "pi" => std::f64::consts::PI,
            "e" => std::f64::consts::E,
            "tau" => std::f64::consts::TAU,
        }
        .unwrap()
    }

    /// To make it more clear to the user what values should be adjusted, the
    /// matrix is updated using the lines on the canvas. All the edges are
    /// considered to be undirected. Anywhere a line exists between a pair of
    /// nodes (i,j) the matrix will show a complex number 1+1i.
    pub(crate) fn update_from_canvas_edges(&mut self, edges: &Vec<(usize, usize)>) {
        let matrix = &mut self.combined_matrix;

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
    combined_matrix: Vec<(f64, f64)>,
    scatter_matrix: Vec<(f64, f64)>,
    propagation_matrix: Vec<(f64, f64)>,
    adjacency_list: HashMap<usize, Vec<usize>>,
    index_map: HashMap<(usize, usize), usize>,
    labels: Vec<(usize, usize)>,
    text_fields: Vec<Vec<Vec<(String, String)>>>,
}
impl From<ComplexMatrixEditor> for SerializedMatrixEditor {
    fn from(m: ComplexMatrixEditor) -> Self {
        Self {
            size: m.combined_matrix.nrows(),
            combined_matrix: m.combined_matrix.iter().map(|x| (x.re, x.im)).collect(),
            scatter_matrix: m.scatter_matrix.iter().map(|x| (x.re, x.im)).collect(),
            propagation_matrix: m.propagation_matrix.iter().map(|x| (x.re, x.im)).collect(),
            adjacency_list: m.adjacency_list,
            index_map: m.index_map,
            labels: m.labels,
            text_fields: m.text_fields,
        }
    }
}
impl From<SerializedMatrixEditor> for ComplexMatrixEditor {
    fn from(m: SerializedMatrixEditor) -> Self {
        Self {
            combined_matrix: DMatrix::from_vec(
                m.size,
                m.size,
                m.combined_matrix
                    .iter()
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect(),
            ),
            scatter_matrix: DMatrix::from_vec(
                m.size,
                m.size,
                m.scatter_matrix
                    .iter()
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect(),
            ),
            propagation_matrix: DMatrix::from_vec(
                m.size,
                m.size,
                m.propagation_matrix
                    .iter()
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect(),
            ),
            math_constants: Self::get_math_constants(),
            adjacency_list: m.adjacency_list,
            index_map: m.index_map,
            labels: m.labels,
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
