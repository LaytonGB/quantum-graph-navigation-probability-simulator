use strum::{Display, VariantArray};

#[derive(Debug, Clone, Copy, Display, VariantArray, serde::Serialize, serde::Deserialize)]
pub enum PropagationMethod {
    ExampleMatrix,
}
