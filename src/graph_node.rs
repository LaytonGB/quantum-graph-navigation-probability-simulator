#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, serde::Serialize, serde::Deserialize)]
pub struct GraphNode {
    pub x: f64,
    pub y: f64,
}

impl GraphNode {
    pub fn new(x: f64, y: f64) -> Self {
        GraphNode { x, y }
    }
}

impl Into<GraphNode> for [f64; 2] {
    fn into(self) -> GraphNode {
        GraphNode {
            x: self[0],
            y: self[1],
        }
    }
}

impl Into<GraphNode> for (f64, f64) {
    fn into(self) -> GraphNode {
        GraphNode {
            x: self.0,
            y: self.1,
        }
    }
}

impl Into<[f64; 2]> for GraphNode {
    fn into(self) -> [f64; 2] {
        [self.x, self.y]
    }
}
