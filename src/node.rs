use crate::position::Position;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub position: Position,
}
