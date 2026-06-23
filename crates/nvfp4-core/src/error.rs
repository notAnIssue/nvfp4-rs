#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("shape mismatched: {0}")]
    ShapeMismatch(String),
    #[error("Unsupported feature: {0}")]
    Unsupported(String),
    #[error("cuda error: {0}")]
    Cuda(String),
}
