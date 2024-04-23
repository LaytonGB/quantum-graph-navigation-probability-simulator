use std::{cell::RefCell, rc::Rc};

use egui_plot::PlotPoints;

use crate::graph_node::GraphNode;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct GraphLine {
    pub start: Rc<RefCell<GraphNode>>,
    pub end: Rc<RefCell<GraphNode>>,
}

impl GraphLine {
    pub fn new(start_node: Rc<RefCell<GraphNode>>, end_node: Rc<RefCell<GraphNode>>) -> Self {
        Self {
            start: start_node,
            end: end_node,
        }
    }

    pub fn other(&self, node: Rc<RefCell<GraphNode>>) -> Rc<RefCell<GraphNode>> {
        if node == self.start {
            self.end.clone()
        } else {
            self.start.clone()
        }
    }

    pub fn is_attached(&self, other: &GraphNode) -> bool {
        self.start.borrow().clone() == *other || self.end.borrow().clone() == *other
    }

    pub fn float_mul(self, rhs: f64) -> (GraphNode, GraphNode) {
        let (a, b) = (self.start.borrow().clone(), self.end.borrow().clone());
        (
            GraphNode::new_unlabelled(a.x * rhs, a.y * rhs),
            GraphNode::new_unlabelled(b.x * rhs, b.y * rhs),
        )
    }

    pub fn closest_point_to_node(&self, p: &GraphNode) -> GraphNode {
        let (a, b) = (self.start.borrow().clone(), self.end.borrow().clone());
        let ab = b - a.clone();
        let ap = (*p).clone() - a.clone();
        let ax = ab.float_mul(ap.dot(&ab) / ab.dot(&ab));
        a + ax
    }

    pub fn closest_endpoint_to_node(&self, p: &GraphNode) -> Option<(GraphNode, f64)> {
        let len = self.len();
        let (a, b) = (self.start.borrow(), self.end.borrow());
        let (ap, bp) = (a.dist(p), b.dist(p));
        if len < ap && len < bp {
            None
        } else if ap <= bp {
            Some((a.clone(), ap))
        } else {
            Some((b.clone(), bp))
        }
    }

    pub fn distance_to_node(&self, p: &GraphNode) -> f64 {
        let n = self.closest_point_to_node(p);
        n.dist(p)
    }

    pub fn dist_squared(a: GraphNode, b: GraphNode) -> f64 {
        (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
    }

    pub fn len_squared(&self) -> f64 {
        let (a, b) = (self.start.borrow(), self.end.borrow());
        (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
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

impl PartialEq for GraphLine {
    fn eq(&self, other: &Self) -> bool {
        Rc::<RefCell<GraphNode>>::ptr_eq(&self.start, &other.start)
            && Rc::<RefCell<GraphNode>>::ptr_eq(&self.end, &other.end)
    }
}

impl Eq for GraphLine {}

impl From<GraphLine> for PlotPoints {
    fn from(val: GraphLine) -> Self {
        let (a, b) = (val.start.borrow(), val.end.borrow());
        vec![[a.x, a.y], [b.x, b.y]].into()
    }
}
