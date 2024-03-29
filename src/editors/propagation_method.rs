use strum::{Display, EnumString, FromRepr, VariantArray};

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    VariantArray,
    EnumString,
    FromRepr,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum PropagationMethod {
    ExampleMatrix,
}
