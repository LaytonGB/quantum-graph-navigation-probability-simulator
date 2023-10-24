use std::rc::Rc;

use egui_plot::PlotPoints;

use crate::GraphNode;

#[derive(Clone, PartialEq, Debug)]
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
}

impl Into<PlotPoints> for GraphLine {
    fn into(self) -> PlotPoints {
        vec![[self.start.x, self.start.y], [self.end.x, self.end.y]].into()
    }
}
