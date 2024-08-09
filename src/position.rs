#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
