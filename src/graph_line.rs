use crate::GraphNode;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GraphLine<'a> {
    pub start: &'a GraphNode,
    pub end: &'a GraphNode,
}

impl<'a> GraphLine<'a> {
    fn other(&self, node: &GraphNode) -> &GraphNode {
        if node == self.start {
            self.end
        } else {
            self.start
        }
    }
}
