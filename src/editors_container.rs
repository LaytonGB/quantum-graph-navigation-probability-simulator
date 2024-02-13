use nalgebra::DVector;

use crate::{classical_state_manager::ClassicalStateManager, editors::MatrixEditor};

#[derive(Debug, Default)]
pub struct EditorsContainer {
    matrix_editor: Option<MatrixEditor>,

    classical_state_manager: Option<ClassicalStateManager>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializedEditorsContainer {
    matrix_editor: Option<MatrixEditor>,
}

impl From<&EditorsContainer> for SerializedEditorsContainer {
    fn from(editors_container: &EditorsContainer) -> Self {
        Self {
            matrix_editor: editors_container.matrix_editor.clone(),
        }
    }
}

impl From<SerializedEditorsContainer> for EditorsContainer {
    fn from(serialized_editors_container: SerializedEditorsContainer) -> Self {
        Self {
            matrix_editor: serialized_editors_container.matrix_editor,
            ..Default::default()
        }
    }
}

impl serde::Serialize for EditorsContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = SerializedEditorsContainer::from(self);
        data.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for EditorsContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = SerializedEditorsContainer::deserialize(deserializer)?;
        Ok(EditorsContainer::from(data))
    }
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

        if let None = self.classical_state_manager {
            if let Ok(csm) =
                ClassicalStateManager::try_from(&self.matrix_editor.as_ref().unwrap().matrix)
            {
                self.classical_state_manager = Some(csm);
            }
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

    pub fn step_state_forward(&mut self) {
        if let Some(manager) = self.classical_state_manager.as_mut() {
            manager.step_forward();
        }
    }

    pub fn clear_all(&mut self) {
        self.matrix_editor = None;
        self.classical_state_manager = None;
    }

    pub(crate) fn get_state_data(&self) -> Option<DVector<f64>> {
        self.classical_state_manager
            .as_ref()
            .and_then(|csm| Some(csm.get_state_data()))
    }

    pub(crate) fn reset_state(&mut self) {
        if let Some(csm) = self.classical_state_manager.as_mut() {
            csm.reset_state();
        }
    }
}
