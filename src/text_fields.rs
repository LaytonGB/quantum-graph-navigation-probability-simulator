#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TextFields {
    pub add_graph_x: String,
    pub add_graph_y: String,
}