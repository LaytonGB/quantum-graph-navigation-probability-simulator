#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    strum::Display,
    strum::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum SimulationMode {
    #[default]
    SideBySide,
    Classical,
    Quantum,
}
