use anyhow::Result;
use nalgebra::DVector;

use crate::{
    classical_state_manager::ClassicalStateManager,
    editors::{matrix_editor::MatrixEditor, ClassicalMatrixEditor, ComplexMatrixEditor, Editor},
    options::Options,
};

#[derive(Debug, Default)]
pub struct EditorsContainer {
    matrix_editor: MatrixEditor,

    classical_state_manager: Option<ClassicalStateManager>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializedEditorsContainer {
    matrix_editor: MatrixEditor,
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
    // TODO temporarily only using classical with updated MatrixEditor enum
    pub fn show_all_editors(&mut self, ui: &mut egui::Ui, size: usize) {
        if let MatrixEditor::Classical(matrix_editor) = &mut self.matrix_editor {
            if matrix_editor.matrix.nrows() < size {
                matrix_editor.resize_matrix(size);
            }
            matrix_editor.show(ui);
        } else {
            self.matrix_editor = MatrixEditor::Classical(ClassicalMatrixEditor::new(size));
        }

        // TODO refactor, this is written very unclearly
        if let None = self.classical_state_manager {
            if let MatrixEditor::Classical(matrix_editor) = &self.matrix_editor {
                if let Ok(csm) = ClassicalStateManager::try_from(&matrix_editor.matrix) {
                    self.classical_state_manager = Some(csm);
                }
            }
        } else if let MatrixEditor::Classical(me) = &self.matrix_editor {
            let csm = self.classical_state_manager.as_mut().unwrap();
            csm.set_transition_matrix_from(&me.matrix);
        }

        self.show_state_details(ui);
        self.show_state_buttons(ui);
    }

    fn show_state_details(&self, ui: &mut egui::Ui) {
        if let Some(csm) = self.classical_state_manager.as_ref() {
            ui.label(format!("Step: {:?}", csm.get_step()));
        }
    }

    fn show_state_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Step").clicked() {
                if let Err(e) = self.step_state_forward() {
                    eprintln!("Error stepping state forward, this normally happens when a user clicks Step without deselecting the matrix editor: {}", e);
                }
            }
            if ui.button("Reset").clicked() {
                self.reset_state();
            }
        });
    }

    pub fn get_matrix_editor(&self) -> &MatrixEditor {
        &self.matrix_editor
    }

    pub fn get_matrix_editor_mut(&mut self) -> &mut MatrixEditor {
        &mut self.matrix_editor
    }

    pub fn remove_nodes(&mut self, node_idxs: Vec<usize>) {
        match &mut self.matrix_editor {
            MatrixEditor::Classical(matrix_editor) => matrix_editor.remove_node(node_idxs),
            _ => (),
        }
    }

    pub fn step_state_forward(&mut self) -> Result<()> {
        if let Some(manager) = self.classical_state_manager.as_mut() {
            manager.step_forward()
        } else {
            Err(anyhow::anyhow!("No state manager found"))
        }
    }

    pub fn clear_all(&mut self) {
        self.matrix_editor = MatrixEditor::None;
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

    // TODO cover classical and complex cases
    pub(crate) fn sync_editors(&mut self, options: &Options, nnodes: usize) {
        match (
            &mut self.matrix_editor,
            self.classical_state_manager.as_mut(),
        ) {
            (MatrixEditor::Classical(me), Some(csm)) => {
                csm.set_start_node_idx(options.generic.start_node_idx);
                if me.is_canvas_update_ready() || !csm.is_transition_matrix_sized_correctly(nnodes)
                {
                    csm.set_transition_matrix_from(&me.matrix);
                }
            }
            _ => (),
        };
    }
}
