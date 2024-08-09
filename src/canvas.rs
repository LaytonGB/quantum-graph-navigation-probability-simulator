use std::{cell::RefCell, rc::Rc};

use crate::{node::Node, position::Position, EframeApp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Canvas {
    pub nodes: Vec<Rc<RefCell<Node>>>,
    pub lines: Vec<(Rc<RefCell<Node>>, Rc<RefCell<Node>>)>,
}

impl serde::Serialize for Canvas {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Canvas {
            nodes: ref_nodes,
            lines: ref_lines,
        } = self;
        let nodes: Vec<Node> = ref_nodes.iter().map(|n| n.borrow().clone()).collect();
        let lines: Vec<(usize, usize)> = ref_lines
            .iter()
            .map(|(n1, n2)| {
                (
                    ref_nodes.iter().position(|node| *node == *n1).unwrap(),
                    ref_nodes.iter().position(|node| *node == *n2).unwrap(),
                )
            })
            .collect();
        let canvas_details = SerializableCanvas { nodes, lines };
        canvas_details.serialize(serializer)
    }
}

impl From<SerializableCanvas> for Canvas {
    fn from(canvas_details: SerializableCanvas) -> Self {
        let SerializableCanvas { nodes, lines } = canvas_details;
        let nodes: Vec<Rc<RefCell<Node>>> = nodes
            .iter()
            .map(|n| Rc::new(RefCell::new(n.clone())))
            .collect();
        let lines: Vec<(Rc<RefCell<Node>>, Rc<RefCell<Node>>)> = lines
            .iter()
            .map(|(n1, n2)| (Rc::clone(&nodes[*n1]), Rc::clone(&nodes[*n2])))
            .collect();
        Self { nodes, lines }
    }
}

impl Canvas {
    #[cfg(target_family = "windows")]
    pub fn place_graph(
        &mut self,
        canvas: &mut Canvas,
        graph_center: Position,
    ) -> Result<(), String> {
        use wfd::DialogParams;

        let loaded_app: EframeApp = wfd::open_dialog(DialogParams {
            file_types: vec![("JSON Files", "*.json")],
            ..Default::default()
        })
        .map_err(|e| format!("File load failed:\n{:?}", e))
        .and_then(|dialog_result| {
            std::fs::read(dialog_result.selected_file_path)
                .map_err(|e| format!("File load failed:\n{}", e))
        })
        .and_then(|file| {
            serde_json::from_slice::<EframeApp>(file.as_slice())
                .map_err(|e| format!("File load failed:\n{}", e))
        })?;
        let canvas_details = SerializableCanvas::from(loaded_app);
        canvas_details.place_on_canvas(canvas, graph_center); // TODO move this function

        Ok(())
    }
}

/// Used to store the canvas nodes and lines when serializing and deserializing.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct SerializableCanvas {
    pub nodes: Vec<Node>,
    pub lines: Vec<(usize, usize)>,
}

impl From<EframeApp> for SerializableCanvas {
    fn from(app: EframeApp) -> Self {
        let Canvas {
            nodes: ref_nodes,
            lines: ref_lines,
        } = app.model.canvas;
        let nodes: Vec<Node> = ref_nodes.iter().map(|n| n.borrow().clone()).collect();
        let lines: Vec<(usize, usize)> = ref_lines
            .iter()
            .map(|(n1, n2)| {
                (
                    ref_nodes.iter().position(|node| *node == *n1).unwrap(),
                    ref_nodes.iter().position(|node| *node == *n2).unwrap(),
                )
            })
            .collect();
        Self { nodes, lines }
    }
}
