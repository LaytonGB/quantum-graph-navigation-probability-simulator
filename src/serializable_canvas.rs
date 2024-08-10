use crate::{canvas::Canvas, node::Node, EframeApp};

/// Used to store the canvas nodes and lines when serializing and deserializing.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SerializableCanvas {
    pub nodes: Vec<Node>,
    pub lines: Vec<(usize, usize)>,
}

impl From<EframeApp> for SerializableCanvas {
    fn from(app: EframeApp) -> Self {
        let Canvas {
            nodes: ref_nodes,
            lines: ref_lines,
            ..
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
