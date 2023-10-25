use std::rc::Rc;

use egui_plot::PlotPoints;

use crate::GraphNode;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct GraphLine {
    pub start: Rc<GraphNode>,
    pub end: Rc<GraphNode>,
}

impl GraphLine {
    pub fn new(start_node: Rc<GraphNode>, end_node: Rc<GraphNode>) -> Self {
        Self {
            start: start_node,
            end: end_node,
        }
    }

    pub fn other(&self, node: Rc<GraphNode>) -> Rc<GraphNode> {
        if node == self.start {
            self.end.clone()
        } else {
            self.start.clone()
        }
    }

    pub fn dist(&self, point: &GraphNode) -> f64 {
        let l = self.len_squared();
        let u = (*self.start).clone() - (*self.end).clone();
        let t = (point.clone() - (*self.end).clone()).dot(&u) / l;
        let t = 0_f64.max(1_f64.min(t));
        GraphLine::dist_squared(
            point.clone(),
            GraphNode::new(self.start.x + t * u.x, self.start.y + t * u.y),
        )
    }

    pub fn dist_squared(a: GraphNode, b: GraphNode) -> f64 {
        (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
    }

    pub fn len_squared(&self) -> f64 {
        Self::dist_squared((*self.start).clone(), (*self.end).clone())
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
}

impl PartialOrd for GraphLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.len_squared().partial_cmp(&other.len_squared())
    }
}

impl Ord for GraphLine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for GraphLine {
    fn eq(&self, other: &Self) -> bool {
        Rc::<GraphNode>::ptr_eq(&self.start, &other.start)
            && Rc::<GraphNode>::ptr_eq(&self.end, &other.end)
    }
}

impl Eq for GraphLine {}

impl Into<PlotPoints> for GraphLine {
    fn into(self) -> PlotPoints {
        vec![[self.start.x, self.start.y], [self.end.x, self.end.y]].into()
    }
}
