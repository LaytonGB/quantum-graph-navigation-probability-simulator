use std::{cell::RefCell, rc::Rc};

use egui_plot::PlotPoints;

use crate::node::Node;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Line {
    pub start: Rc<RefCell<Node>>,
    pub end: Rc<RefCell<Node>>,
}

impl From<(Rc<RefCell<Node>>, Rc<RefCell<Node>>)> for Line {
    fn from((start, end): (Rc<RefCell<Node>>, Rc<RefCell<Node>>)) -> Self {
        Self { start, end }
    }
}

impl From<Line> for PlotPoints {
    fn from(line: Line) -> Self {
        let Line { start, end } = line;
        let (start, end) = (start.borrow(), end.borrow());
        Self::new(vec![start.position.into(), end.position.into()])
    }
}
