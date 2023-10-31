use egui::Ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct ComputationOptions {
    pub style: ComputationStyle,
    pub specific: ComputationOption,
    pub generic: GenericComputationOptions,
}

impl ComputationOptions {
    pub fn show_style_buttons(&mut self, ui: &mut Ui) {
        for style in [ComputationStyle::Classical, ComputationStyle::Quantum] {
            let mut btn = ui.button(format!("{}", style.name()));
            if style == self.style {
                btn = btn.highlight();
            }

            if btn.clicked() {
                self.style = style;
            }
        }
    }

    pub fn show_specific_options(&self, _ui: &mut Ui) {
        match self.specific {
            ComputationOption::Classical(_classical_options) => {}
            ComputationOption::Quantum(_quantum_options) => {}
        }
    }

    pub fn show_generic_options(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.generic.revisit_same_node, "Allow self-traversal");
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ComputationStyle {
    #[default]
    Classical,
    Quantum,
}

impl ComputationStyle {
    pub fn name(&self) -> &'static str {
        match self {
            ComputationStyle::Classical => "Classical",
            ComputationStyle::Quantum => "Quantum",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ComputationOption {
    Classical(ClassicalOptions),
    Quantum(QuantumOptions),
}

impl Default for ComputationOption {
    fn default() -> Self {
        Self::Classical(ClassicalOptions::default())
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
