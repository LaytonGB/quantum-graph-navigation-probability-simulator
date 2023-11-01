use egui::{Color32, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Options {
    pub mode: Mode,
    pub specific: ModeOptions,
    pub generic: GenericComputationOptions,
}

impl Options {
    pub fn show_mode_buttons(&mut self, ui: &mut Ui) {
        for mode in [Mode::Edit, Mode::Classical, Mode::Quantum] {
            let mut btn = ui.button(format!("{}", mode.name()));
            if mode == self.mode {
                btn = btn.highlight();
            }

            if btn.clicked() {
                self.mode = mode;
            }
        }
    }

    pub fn show_specific_options(&mut self, ui: &mut Ui) {
        match self.mode {
            Mode::Edit => self.specific.edit.draw_snap_options(ui),
            Mode::Classical => {}
            Mode::Quantum => {}
        }
    }

    pub fn show_generic_options(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.generic.revisit_same_node, "Allow self-traversal");
    }

    pub fn get_node_color(&self) -> Color32 {
        match self.mode {
            Mode::Edit => Color32::RED,
            Mode::Classical => Color32::GRAY,
            Mode::Quantum => Color32::GRAY,
        }
    }

    pub fn get_line_color(&self) -> Color32 {
        match self.mode {
            Mode::Edit => Color32::BLUE,
            Mode::Classical => Color32::GRAY,
            Mode::Quantum => Color32::GRAY,
        }
    }
}

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct ClassicalOptions {}

#[derive(Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct QuantumOptions {}

#[derive(Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub struct GenericComputationOptions {
    revisit_same_node: bool,
}
