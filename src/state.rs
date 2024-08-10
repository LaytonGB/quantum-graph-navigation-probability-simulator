use std::path::PathBuf;

use crate::{position::Position, simulation_mode::SimulationMode, tool::Tool};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SimulationState {
    Paused { step: usize },
    Running { step: usize },
}

impl Default for SimulationState {
    fn default() -> Self {
        Self::Paused { step: 0 }
    }
}

#[derive(Debug, Clone, strum::Display, serde::Serialize, serde::Deserialize)]
pub enum State {
    Editing {
        selected_tool: Tool,
    },
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

impl Default for State {
    fn default() -> Self {
        Self::Editing {
            selected_tool: Tool::Node,
        }
    }
}
