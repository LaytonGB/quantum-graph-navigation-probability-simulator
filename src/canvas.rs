use std::{cell::RefCell, rc::Rc};

use crate::{node::Node, serializable_canvas::SerializableCanvas};

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
        let lines: Vec<(Rc<RefCell<Node>>, Rc<RefCell<Node>>)> = lines
            .iter()
            .map(|(n1, n2)| (Rc::clone(&nodes[*n1]), Rc::clone(&nodes[*n2])))
            .collect();
        Self { nodes, lines }
    }
}
