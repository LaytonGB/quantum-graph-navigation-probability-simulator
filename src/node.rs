use crate::position::Position;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub position: Position,
}
