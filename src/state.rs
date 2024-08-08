#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SimulationMode {
    #[default]
    Classical,
    Quantum,
    SideBySide,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SimulationState {
    Paused(usize),
    Running(usize),
}

impl Default for SimulationState {
    fn default() -> Self {
        Self::Paused(0)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Editing,
    Simulating {
        mode: SimulationMode,
        state: SimulationState,
    },
}
