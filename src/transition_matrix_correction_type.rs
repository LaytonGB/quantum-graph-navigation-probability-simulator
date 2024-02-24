use nalgebra::DVector;

#[derive(Debug, PartialEq, Clone)]
pub enum TransitionMatrixCorrectionType {
    None,
    Scalar(f64),
    NonScalar(DVector<f64>),
}

impl TransitionMatrixCorrectionType {
    pub fn to_str(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Scalar(x) => format!("scalar {}", x),
            Self::NonScalar(x) => format!("non_scalar {}", x),
        }
    }
}
