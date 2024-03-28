use anyhow::Result;
use nalgebra::DVector;

use crate::{
    editors::{
        matrix_editor::MatrixEditor, state_manager::StateManager, ClassicalMatrixEditor,
        ClassicalStateManager, ComplexMatrixEditor, ComplexStateManager, Editor,
    },
    options::Options,
};

#[derive(Debug, Default)]
pub struct EditorsContainer {
    matrix_editor: MatrixEditor,

    state_manager: StateManager,
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

// TODO reduce the number of calls made during show editors
impl EditorsContainer {
    pub fn show_classical_editors(&mut self, ui: &mut egui::Ui, node_count: usize) {
        if !self.matrix_editor.is_classical() {
            self.matrix_editor = MatrixEditor::Classical(ClassicalMatrixEditor::new(node_count));
        }

        if let MatrixEditor::Classical(cme) = &mut self.matrix_editor {
            if cme.matrix.nrows() < node_count {
                cme.resize_matrix(node_count);
            }
            cme.show(ui);

            if let StateManager::Classical(csm) = &mut self.state_manager {
                csm.set_transition_matrix_from(&cme.matrix);
            } else {
                match ClassicalStateManager::try_from(&cme.matrix) {
                    Ok(csm) => self.state_manager = StateManager::Classical(csm),
                    Err(e) => eprintln!("Error converting matrix to state manager: {}", e),
                }
            }
        }

        self.show_state_details(ui);
        self.show_state_buttons(ui);
    }

    pub fn show_quantum_editors(&mut self, ui: &mut egui::Ui, edges: &Vec<(usize, usize)>) {
        if !self.matrix_editor.is_complex() {
            self.matrix_editor = MatrixEditor::Complex(ComplexMatrixEditor::new(edges));
        }

        if let MatrixEditor::Complex(cme) = &mut self.matrix_editor {
            cme.show(ui);

            if let StateManager::Complex(csm) = &mut self.state_manager {
                csm.set_transition_matrix_from(&cme.combined_matrix);
            } else if let Ok(csm) = ComplexStateManager::try_from(&cme.combined_matrix) {
                self.state_manager = StateManager::Complex(csm);
            }
        }

        self.show_state_details(ui);
        self.show_state_buttons(ui);
    }

    fn show_state_details(&self, ui: &mut egui::Ui) {
        match &self.state_manager {
            StateManager::None => (),
            StateManager::Classical(csm) => {
                ui.label(format!("Step: {:?}", csm.get_step()));
            }
            StateManager::Complex(csm) => {
                ui.label(format!("Step: {:?}", csm.get_step()));
            }
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

    pub fn remove_nodes(&mut self, node_indexes: Vec<usize>) {
        match &mut self.matrix_editor {
            MatrixEditor::Classical(matrix_editor) => matrix_editor.remove_node(node_indexes),
            MatrixEditor::Complex(_) => self.matrix_editor = MatrixEditor::None,
            _ => (),
        }
    }

    pub fn step_state_forward(&mut self) -> Result<()> {
        match &mut self.state_manager {
            StateManager::Classical(csm) => csm.step_forward(),
            StateManager::Complex(csm) => csm.step_forward(),
            StateManager::None => Err(anyhow::anyhow!("No state manager found")),
        }
    }

    pub fn clear_all(&mut self) {
        self.matrix_editor = MatrixEditor::None;
        self.state_manager = StateManager::None;
    }

    pub(crate) fn get_state_data(&self) -> Option<DVector<f64>> {
        match &self.state_manager {
            StateManager::Classical(csm) => Some(csm.get_state_data()),
            StateManager::Complex(csm) => Some(csm.get_state_data()),
            _ => None,
        }
    }

    pub(crate) fn reset_state(&mut self) {
        match &mut self.state_manager {
            StateManager::Classical(csm) => {
                if let MatrixEditor::Classical(cme) = &self.matrix_editor {
                    csm.reset_state(&cme.matrix);
                } else {
                    panic!("State manager is classical but matrix editor is not");
                }
            }
            StateManager::Complex(csm) => csm.reset_state(),
            StateManager::None => (),
        }
    }

    pub(crate) fn update_editor_from_edges(&mut self, edges: &Vec<(usize, usize)>) {
        match &mut self.matrix_editor {
            MatrixEditor::Classical(me) => me.update_from_canvas_edges(edges),
            MatrixEditor::Complex(me) => me.update_from_canvas_edges(edges),
            _ => (),
        }
    }

    pub(crate) fn sync_editors(&mut self, options: &Options, node_count: usize) {
        match (&mut self.matrix_editor, &mut self.state_manager) {
            (MatrixEditor::Classical(me), StateManager::Classical(csm)) => {
                csm.set_start_node_idx(options.generic.start_node_idx);
                if me.is_canvas_update_ready()
                    || !csm.is_transition_matrix_sized_correctly(node_count)
                {
                    csm.set_transition_matrix_from(&me.matrix);
                }
            }
            (MatrixEditor::Complex(me), StateManager::Complex(csm)) => {
                csm.set_start_node_idx(options.generic.start_node_idx);
                if me.is_canvas_update_ready()
                    || !csm.is_transition_matrix_sized_correctly(node_count)
                {
                    csm.set_transition_matrix_from(&me.combined_matrix);
                }
            }
            _ => (),
        };
    }
}
