use std::collections::HashSet;

use egui::{Color32, Ui};

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Options {
    pub mode: Mode,
    pub mode_change_data: Option<(Mode, Mode)>,

    pub specific: ModeOptions,

    pub generic: GenericComputationOptions,
}

impl Options {
    pub fn show_mode_buttons(&mut self, ui: &mut Ui) {
        ui.heading("Computation Style");
        for mode in [Mode::Edit, Mode::Classical, Mode::Quantum] {
            let mut btn = ui.button(format!("{}", mode.name()));
            if mode == self.mode {
                btn = btn.highlight();
            }

            if btn.clicked() {
                self.set_mode(mode);
            }
        }
    }

    pub fn show_specific_options(&mut self, ui: &mut Ui) {
        ui.heading(self.mode.options_name());
        match self.mode {
            Mode::Edit => self.specific.edit.show_options(ui),
            Mode::Classical => self.specific.classical.show_options(ui),
            Mode::Quantum => self.specific.quantum.show_options(ui),
        }
    }

    pub fn show_generic_options(&mut self, ui: &mut Ui) {
        ui.heading("Generic Options");
        ui.horizontal(|ui| {
            ui.label("Start Node Index");
            if ui
                .text_edit_singleline(&mut self.generic.start_node_idx_text_field)
                .changed()
            {
                // FIXME apply more checks here, node must be within range
                if let Ok(idx) = self.generic.start_node_idx_text_field.parse::<usize>() {
                    self.generic.start_node_idx = idx;
                } else {
                    self.generic.start_node_idx_text_field =
                        self.generic.previous_start_node_idx_text_field.clone();
                }
            }
        });
    }

    pub fn get_node_color(&self) -> Color32 {
        Color32::RED
    }

    pub fn get_line_color(&self) -> Color32 {
        match self.mode {
            Mode::Edit => Color32::BLUE,
            Mode::Classical => Color32::WHITE,
            Mode::Quantum => Color32::WHITE,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        if self.mode == mode {
            return;
        }

        self.mode_change_data = Some((self.mode, mode));
        self.mode = mode;
    }

    pub fn clear_mode_change_data(&mut self) {
        self.mode_change_data = None;
    }
}

// TODO use [`strum`] here
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Mode {
    #[default]
    Edit,
    Classical,
    Quantum,
}

impl Mode {
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Edit => "Edit",
            Mode::Classical => "Classical",
            Mode::Quantum => "Quantum",
        }
    }

    pub fn options_name(&self) -> &'static str {
        match self {
            Mode::Edit => "Editing Options",
            Mode::Classical => "Simulation Options (Classical)",
            Mode::Quantum => "Simulation Options (Quantum)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct ModeOptions {
    pub edit: EditOptions,
    pub classical: ClassicalOptions,
    pub quantum: QuantumOptions,
}

trait ModeOptionsShow {
    fn show_options(&mut self, ui: &mut Ui);
}

#[derive(Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, Debug)]
pub enum Snap {
    #[default]
    None,
    Half,
    One,
    Five,
    Ten,
}

impl Snap {
    fn as_num_str(&self) -> String {
        match self {
            Snap::None => "None",
            Snap::Half => "0.5",
            Snap::One => "1.0",
            Snap::Five => "5.0",
            Snap::Ten => "10.0",
        }
        .to_owned()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct EditOptions {
    pub snap: Snap,
}

impl EditOptions {
    fn draw_snap_options(&mut self, ui: &mut Ui) {
        ui.label("Snapping Increment");
        ui.group(|ui| {
            ui.horizontal(|ui| {
                for snap in [Snap::None, Snap::Half, Snap::One, Snap::Five, Snap::Ten] {
                    ui.radio_value(&mut self.snap, snap, snap.as_num_str());
                }
            });
        });
    }
}

impl ModeOptionsShow for EditOptions {
    fn show_options(&mut self, ui: &mut Ui) {
        self.draw_snap_options(ui);
    }
}

#[derive(Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct ClassicalOptions {}

impl ModeOptionsShow for ClassicalOptions {
    fn show_options(&mut self, _ui: &mut Ui) {}
}

#[derive(Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct QuantumOptions {
    target_node_text: String,
    pub target_node_indexes: HashSet<usize>,
}

impl QuantumOptions {
    fn update_target_node_indexes(&mut self) {
        self.target_node_indexes = self
            .target_node_text
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();
    }
}

impl ModeOptionsShow for QuantumOptions {
    fn show_options(&mut self, ui: &mut Ui) {
        ui.label("Target Node Indexes (space separated)");
        if ui
            .text_edit_singleline(&mut self.target_node_text)
            .lost_focus()
        {
            self.update_target_node_indexes();
        }
    }
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct GenericComputationOptions {
    pub start_node_idx: usize,
    pub start_node_idx_text_field: String,
    pub previous_start_node_idx_text_field: String,
}

impl Default for GenericComputationOptions {
    fn default() -> Self {
        Self {
            start_node_idx: Default::default(),
            start_node_idx_text_field: String::from("0"),
            previous_start_node_idx_text_field: String::from("0"),
        }
    }
}
