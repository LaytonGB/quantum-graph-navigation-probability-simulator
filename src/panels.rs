#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Layout {
    pub tools: bool,
    pub mode: bool,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            tools: true,
            mode: true,
        }
    }
}
