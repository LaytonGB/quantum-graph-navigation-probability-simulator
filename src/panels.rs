#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Panels {
    pub tools: bool,
    pub mode: bool,
}

impl Default for Panels {
    fn default() -> Self {
        Self {
            tools: true,
            mode: true,
        }
    }
}
