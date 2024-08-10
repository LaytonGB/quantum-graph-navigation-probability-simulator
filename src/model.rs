use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{
    canvas::Canvas, node::Node, panels::Panels, position::Position,
    serializable_canvas::SerializableCanvas, state::State, text_fields::TextFields, EframeApp,
};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Model {
    pub state: State,
    pub text_fields: TextFields,
    pub canvas: Canvas,
    pub panels: Panels,
}

impl Model {
    pub fn save(app: &EframeApp, path_buffer: &PathBuf) -> Result<(), String> {
        std::fs::write(path_buffer, serde_json::to_string(app).unwrap())
            .map_err(|e| format!("File save failed:\n{}", e))
    }

    pub fn load(app: &mut EframeApp, path_buffer: &PathBuf) -> Result<(), String> {
        match std::fs::read(path_buffer)
            .map(|file| serde_json::from_slice::<EframeApp>(file.as_slice()))
        {
            Ok(Ok(saved_app)) => *app = saved_app,
            Ok(Err(e)) => return Err(format!("File load failed:\n{}", e)),
            Err(e) => return Err(format!("File load failed:\n{}", e)),
        }

        Ok(())
    }

    pub fn queue_place_graph(&mut self, path_buffer: PathBuf, position: Position) {
        self.state = State::PendingPlace {
            path_buffer: path_buffer.clone(),
            position: position.clone(),
        };
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.nodes = vec![];
        self.canvas.lines = vec![];
    }
}
