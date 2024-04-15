use std::collections::HashMap;

use evalexpr::{context_map, eval_with_context, HashMapContext, Value};
use nalgebra::{Complex, DMatrix};
use strum::VariantArray as _;

use super::{Editor, PropagationMethod};

#[derive(Debug, Clone)]
pub struct ComplexMatrixEditor {
    scatter_matrix: DMatrix<Complex<f64>>,
    propagation_matrix: DMatrix<Complex<f64>>,
    combined_matrix: DMatrix<Complex<f64>>,

    math_constants: HashMapContext,

    self_traversing_nodes: Vec<bool>,
    adjacency_list: HashMap<usize, Vec<usize>>,
    labels: Vec<(usize, usize)>,

    propagation_method: PropagationMethod,

    /// 3 vectors deep refer to: start node, end node, line of connections
    /// edges: a->b       a->c
    /// a->b   [0][1][0]  [0][2][0]
    /// a->c   [0][1][1]  [0][2][1]
    ///
    /// each node has an N by N matrix of text fields
    /// where N is the number of connections the node has
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

        let labels = Self::new_labels(&adjacency_list);

        let half_edge_count = labels.len();

        // FIXME populate the scatter matrix
        // let nodes_with_edges_count = node_edge_counts.len();
        // for i in 0..nodes_with_edges_count {
        //     scatter_matrix[(i, i)] = Complex::new(1.0, 0.0);
        //     scatter_matrix[(i + 1, i)] = Complex::new(0.0, 1.0);
        //     scatter_matrix[(i, i + 1)] = Complex::new(0.0, 1.0);
        //     scatter_matrix[(i + 1, i + 1)] = Complex::new(1.0, 0.0);
        // }

        let self_traversing_nodes = adjacency_list
            .iter()
            .map(|(k, v)| v.contains(k))
            .collect::<Vec<_>>();

        let propagation_method = PropagationMethod::ExampleMatrix;
        let scatter_matrix = Self::new_scatter_matrix(half_edge_count);
        let propagation_matrix = Self::new_propagation_matrix(propagation_method, &labels);

        let text_fields = Self::new_text_fields(&adjacency_list);

        Self {
            combined_matrix: &scatter_matrix * &propagation_matrix,
            scatter_matrix,
            propagation_matrix,

            math_constants: Self::get_math_constants(),

            self_traversing_nodes,
            adjacency_list,
            labels,

            propagation_method,

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
        ui.separator();
        ui.heading("Preview");
        self.show_preview_fields(ui);
        ui.separator();
    }

    fn show_text_fields(&mut self, ui: &mut egui::Ui) {
        if self.adjacency_list.is_empty() {
            ui.label("No edges to show");
            return;
        }

        // sort nodes into order
        let mut from_nodes = self.adjacency_list.keys().copied().collect::<Vec<_>>();
        from_nodes.sort_unstable();

        // display propagation method from preset selection
        egui::ComboBox::from_label("Propagation method")
            .selected_text(format!("{}", self.propagation_method))
            .show_ui(ui, |ui| {
                for method in PropagationMethod::VARIANTS {
                    if ui.button(format!("{}", method)).clicked() {
                        self.propagation_method = *method;
                        self.propagation_matrix =
                            Self::new_propagation_matrix(self.propagation_method, &self.labels);
                        self.combined_matrix = &self.scatter_matrix * &self.propagation_matrix;
                    }
                }
            });

        // display section for each node's connections
        for (i, from) in from_nodes.iter().enumerate() {
            let connections = self.adjacency_list.get(from).unwrap().clone();
            ui.collapsing(format!("Node {}", from), |ui| {
                if ui
                    .checkbox(
                        &mut self.self_traversing_nodes[*from],
                        "Can traverse to self.",
                    )
                    .changed()
                {
                    let connections = self.adjacency_list.get_mut(from).unwrap();
                    if connections.contains(from) {
                        connections.retain(|x| x != from);
                    } else {
                        connections.push(*from);
                        connections.sort_unstable();
                    }

                    self.reset_from_adjacency_list();
                    return;
                }

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
                                if ui.text_edit_singleline(&mut field.0).lost_focus() {
                                    self.text_fields_modified = true;
                                }

                                if ui.text_edit_singleline(&mut field.1).lost_focus() {
                                    self.text_fields_modified = true;
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        }
    }

    fn apply_text_fields(&mut self) {
        // the position of each group of numbers is the index of the FROM node plus
        // the sum of all previous adjacent nodes

        // sort nodes into order
        let mut past_adjacencies = 0;
        let mut from_nodes = self.adjacency_list.keys().copied().collect::<Vec<_>>();
        from_nodes.sort_unstable();

        // copy data from text fields into scatter matrix
        for &i in from_nodes.iter() {
            let connections = self.adjacency_list.get(&i).unwrap();
            let connections_count = connections.len();

            for j in 0..connections_count {
                for k in 0..connections_count {
                    let re = eval_with_context(&self.text_fields[i][j][k].0, &self.math_constants);
                    let re = match re {
                        Ok(Value::Int(num)) => num as f64,
                        Ok(Value::Float(num)) => num,
                        _ => {
                            self.text_fields[i][j] = self.previous_text_fields[i][j].clone();
                            continue;
                        }
                    };

                    let im = eval_with_context(&self.text_fields[i][j][k].1, &self.math_constants);
                    let im = match im {
                        Ok(Value::Int(num)) => num as f64,
                        Ok(Value::Float(num)) => num,
                        _ => {
                            self.text_fields[i] = self.previous_text_fields[i].clone();
                            continue;
                        }
                    };

                    self.set_value(
                        past_adjacencies + j,
                        past_adjacencies + k,
                        Complex::new(re, im),
                    );
                }
            }

            past_adjacencies += connections_count;
        }

        self.combined_matrix = &self.scatter_matrix * &self.propagation_matrix;
        self.previous_text_fields = self.text_fields.clone();
        self.text_fields_modified = false;
        self.is_canvas_update_ready = true;
    }

    fn set_value(&mut self, row: usize, col: usize, value: Complex<f64>) {
        self.scatter_matrix[(row, col)] = value;
    }

    fn reset_from_adjacency_list(&mut self) {
        self.labels = Self::new_labels(&self.adjacency_list);
        self.text_fields = Self::new_text_fields(&self.adjacency_list);
        self.previous_text_fields = self.text_fields.clone();
        self.scatter_matrix = Self::new_scatter_matrix(self.labels.len());
        self.propagation_matrix =
            Self::new_propagation_matrix(self.propagation_method, &self.labels);
        self.combined_matrix = &self.scatter_matrix * &self.propagation_matrix;
        self.is_canvas_update_ready = true;
    }

    fn new_labels(adjacency_list: &HashMap<usize, Vec<usize>>) -> Vec<(usize, usize)> {
        let mut labels = adjacency_list.iter().collect::<Vec<_>>();
        labels.sort_unstable();
        labels
            .into_iter()
            .flat_map(|(i, v)| v.iter().map(move |j| (*i, *j)))
            .collect()
    }

    fn new_text_fields(
        adjacency_list: &HashMap<usize, Vec<usize>>,
    ) -> Vec<Vec<Vec<(String, String)>>> {
        let mut sorted_node_list = adjacency_list.keys().collect::<Vec<_>>();
        sorted_node_list.sort_unstable();

        sorted_node_list.iter().fold(Vec::new(), |mut v, k| {
            let connections = adjacency_list.get(k).unwrap();
            v.push(vec![
                vec![
                    (String::from("0"), String::from("0"));
                    connections.len()
                ];
                connections.len()
            ]);
            v
        })
    }

    fn new_scatter_matrix(half_edge_count: usize) -> DMatrix<Complex<f64>> {
        // TODO allow customization
        DMatrix::from_element(half_edge_count, half_edge_count, Complex::new(0.0, 0.0))
    }

    fn new_propagation_matrix(
        propagation_method: PropagationMethod,
        labels: &Vec<(usize, usize)>,
    ) -> DMatrix<Complex<f64>> {
        let n = labels.len();
        match propagation_method {
            PropagationMethod::Blank => DMatrix::from_element(n, n, Complex::new(0.0, 0.0)),
            PropagationMethod::ExampleMatrix => {
                DMatrix::from_fn(n, n, |i, j| {
                    // where the coordinates point to some node that has 2 edges, eg 0->0, 0->1
                    // being on some edge 0->1 would then place the particle on edge 1->0
                    // 0, 1
                    // 1, 0
                    if labels[i].0 == labels[j].1 && labels[i].1 == labels[j].0 {
                        Complex::new(1.0, 0.0)
                    } else {
                        Complex::new(0.0, 0.0)
                    }
                })
            }
        }
    }

    fn show_preview_fields(&self, ui: &mut egui::Ui) {
        ui.collapsing("Scatter Matrix", |ui| {
            self.display_matrix(ui, &self.scatter_matrix, "scatter");
        });
        ui.collapsing("Propagation Matrix", |ui| {
            self.display_matrix(ui, &self.propagation_matrix, "propagation");
        });
        ui.collapsing("Combined Matrix", |ui| {
            self.display_matrix(ui, &self.combined_matrix, "combined");
        });
    }

    fn display_matrix(
        &self,
        ui: &mut egui::Ui,
        matrix: &DMatrix<Complex<f64>>,
        preview_id_prefix: &'static str,
    ) {
        if self.labels.len() != matrix.nrows() || self.labels.len() != matrix.ncols() {
            panic!("Matrix dimensions do not match labels")
        }

        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::Grid::new(format!("{}_matrix_preview", preview_id_prefix))
                .striped(true)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    // column headers
                    ui.label(""); // empty label to pad for row headers
                    for l in self.labels.iter() {
                        ui.label(egui::RichText::new(format!("{}->{}", l.0, l.1)).strong());
                    }
                    ui.end_row();

                    // row headers and values
                    for (i, l) in self.labels.iter().enumerate() {
                        ui.label(egui::RichText::new(format!("{}->{}", l.0, l.1)).strong());
                        for j in 0..self.labels.len() {
                            if matrix[(i, j)].l1_norm() == 0.0 {
                                ui.label("-");
                            } else {
                                ui.label(format!("{}+{}i", matrix[(i, j)].re, matrix[(i, j)].im));
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }

    fn get_math_constants() -> HashMapContext {
        context_map! {
            "pi" => std::f64::consts::PI,
            "e" => std::f64::consts::E,
            "tau" => std::f64::consts::TAU,
        }
        .unwrap()
    }

    pub fn get_combined_matrix(&self) -> &DMatrix<Complex<f64>> {
        &self.combined_matrix
    }

    pub fn get_labels(&self) -> &[(usize, usize)] {
        &self.labels
    }

    pub fn get_adjacency_list(&self) -> &HashMap<usize, Vec<usize>> {
        &self.adjacency_list
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SerializedMatrixEditor {
    size: usize,
    combined_matrix: Vec<(f64, f64)>,
    scatter_matrix: Vec<(f64, f64)>,
    propagation_matrix: Vec<(f64, f64)>,
    self_traversing_nodes: Vec<bool>,
    adjacency_list: HashMap<usize, Vec<usize>>,
    labels: Vec<(usize, usize)>,
    propagation_method: PropagationMethod,
    text_fields: Vec<Vec<Vec<(String, String)>>>,
}
impl From<ComplexMatrixEditor> for SerializedMatrixEditor {
    fn from(m: ComplexMatrixEditor) -> Self {
        Self {
            size: m.combined_matrix.nrows(),
            combined_matrix: m.combined_matrix.iter().map(|x| (x.re, x.im)).collect(),
            scatter_matrix: m.scatter_matrix.iter().map(|x| (x.re, x.im)).collect(),
            propagation_matrix: m.propagation_matrix.iter().map(|x| (x.re, x.im)).collect(),
            self_traversing_nodes: m.self_traversing_nodes,
            adjacency_list: m.adjacency_list,
            labels: m.labels,
            propagation_method: m.propagation_method,
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
            self_traversing_nodes: m.self_traversing_nodes,
            adjacency_list: m.adjacency_list,
            labels: m.labels,
            propagation_method: m.propagation_method,
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
