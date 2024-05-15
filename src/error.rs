use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ReferenceError {
    #[error("failed to extract the type from the reference")]
    TypeExtractionFailed,
}
