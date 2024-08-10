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
    pub fn save(&self, app: &EframeApp) -> Result<(), String> {
        let State::PendingSave { ref path_buffer } = self.state else {
            panic!("Can only save when State is PendingSave");
        };

        std::fs::write(path_buffer, serde_json::to_string(app).unwrap())
            .map_err(|e| format!("File save failed:\n{}", e))
    }

    pub fn load(&self, app: &mut EframeApp) -> Result<(), String> {
        let State::PendingLoad { ref path_buffer } = self.state else {
            panic!("Can only save when State is PendingLoad");
        };

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

    #[cfg(target_family = "windows")]
    pub fn place_graph(&mut self) -> Result<(), String> {
        use crate::serializable_canvas::SerializableCanvas;

        let (canvas_details, position): (SerializableCanvas, Position) = {
            let State::PendingPlace {
                ref path_buffer,
                ref position,
            } = self.state
            else {
                panic!("Can only place graph when State is PendingPlace");
            };

            (
                std::fs::read(path_buffer)
                    .map_err(|e| format!("File load failed:\n{}", e))
                    .and_then(|file| {
                        serde_json::from_slice::<EframeApp>(file.as_slice())
                            .map_err(|e| format!("File load failed:\n{}", e))
                    })?
                    .into(),
                *position,
            )
        };

        self.place_on_canvas(canvas_details, position);

        Ok(())
    }

    pub fn place_on_canvas(&mut self, canvas_details: SerializableCanvas, position: Position) {
        let SerializableCanvas { nodes, lines } = canvas_details;
        let nodes: Vec<Rc<RefCell<Node>>> = nodes
            .iter()
            .map(|n| Rc::new(RefCell::new(n.clone())))
            .collect();
        let lines: Vec<(Rc<RefCell<Node>>, Rc<RefCell<Node>>)> = lines
            .iter()
            .map(|(n1, n2)| (Rc::clone(&nodes[*n1]), Rc::clone(&nodes[*n2])))
            .collect();

        let offset = Position::ZERO - position;

        for node in nodes.iter() {
            let mut node = node.borrow_mut();
            node.position += offset;
        }

        self.canvas.nodes.extend(nodes);
        self.canvas.lines.extend(lines);
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.nodes = vec![];
        self.canvas.lines = vec![];
    }
}
