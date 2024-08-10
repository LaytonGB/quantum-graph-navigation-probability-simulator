use std::{cell::RefCell, rc::Rc};

use crate::{
    canvas_state::CanvasState, node::Node, position::Position,
    serializable_canvas::SerializableCanvas,
};

pub type Line = (Rc<RefCell<Node>>, Rc<RefCell<Node>>);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Canvas {
    pub state: CanvasState,
    pub nodes: Vec<Rc<RefCell<Node>>>,
    pub lines: Vec<Line>,
}

impl Canvas {
    pub fn place_on_canvas(&mut self, canvas_details: SerializableCanvas, position: Position) {
        let SerializableCanvas { nodes, lines } = canvas_details;
        let nodes: Vec<Rc<RefCell<Node>>> = nodes
            .iter()
            .map(|n| Rc::new(RefCell::new(n.clone())))
            .collect();
        let lines: Vec<Line> = lines
            .iter()
            .map(|(n1, n2)| (Rc::clone(&nodes[*n1]), Rc::clone(&nodes[*n2])))
            .collect();

        let offset = Position::ZERO - position;

        for node in nodes.iter() {
            let mut node = node.borrow_mut();
            node.position += offset;
        }

        self.nodes.extend(nodes);
        self.lines.extend(lines);
    }
}

impl serde::Serialize for Canvas {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Canvas {
            nodes: ref_nodes,
            lines: ref_lines,
            ..
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

impl<'de> serde::Deserialize<'de> for Canvas {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let canvas_details = SerializableCanvas::deserialize(deserializer)?;
        Ok(canvas_details.into())
    }
}

impl From<SerializableCanvas> for Canvas {
    fn from(canvas_details: SerializableCanvas) -> Self {
        let SerializableCanvas { nodes, lines } = canvas_details;
        let nodes: Vec<Rc<RefCell<Node>>> = nodes
            .iter()
            .map(|n| Rc::new(RefCell::new(n.clone())))
            .collect();
        let lines: Vec<Line> = lines
            .iter()
            .map(|(n1, n2)| (Rc::clone(&nodes[*n1]), Rc::clone(&nodes[*n2])))
            .collect();
        Self {
            nodes,
            lines,
            ..Default::default()
        }
    }
}
