use std::{cell::RefCell, rc::Rc};

use crate::{node::Node, position::Position};

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CanvasState {
    #[default]
    None,
    Dragging {
        node: Rc<RefCell<Node>>,
    },
    PendingNodePlacement {
        position: Position,
    },
    PendingLineStartPlacement {
        node: Rc<RefCell<Node>>,
    },
    PlacingLine {
        start: Rc<RefCell<Node>>,
    },
    PendingLineEndPlacement {
        start: Rc<RefCell<Node>>,
        end: Rc<RefCell<Node>>,
    },
    PendingLineDeletion {
        start: Rc<RefCell<Node>>,
        end: Rc<RefCell<Node>>,
    },
    PendingNodeDeletion {
        node: Rc<RefCell<Node>>,
    },
}
