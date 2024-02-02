use crate::editors::MatrixEditor;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EditorsContainer {
    matrix_editor: Option<MatrixEditor>,
}

impl EditorsContainer {
    pub fn show_matrix_editor(&mut self, ui: &mut egui::Ui, size: usize) {
        if let Some(matrix_editor) = &mut self.matrix_editor {
            if matrix_editor.matrix.nrows() < size {
                matrix_editor.resize_matrix(size);
            }
            matrix_editor.show(ui);
        } else {
            self.matrix_editor = Some(MatrixEditor::new(size));
        }
    }

    pub fn get_matrix_editor(&self) -> Option<&MatrixEditor> {
        self.matrix_editor.as_ref()
    }

    pub fn get_matrix_editor_mut(&mut self) -> Option<&mut MatrixEditor> {
        self.matrix_editor.as_mut()
    }

    pub fn remove_nodes(&mut self, node_idxs: Vec<usize>) {
        if let Some(matrix_editor) = &mut self.matrix_editor {
            matrix_editor.remove_node(node_idxs);
        }
    }

    pub fn clear_all(&mut self) {
        self.matrix_editor = None;
    }
}
