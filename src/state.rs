use std::path::PathBuf;

use crate::position::Position;

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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Editing,
    Simulating {
        mode: SimulationMode,
        state: SimulationState,
    },
    PendingSave {
        path_buffer: PathBuf,
    },
    PendingLoad {
        path_buffer: PathBuf,
    },
    PendingPlace {
        path_buffer: PathBuf,
        position: Position,
    },
}
