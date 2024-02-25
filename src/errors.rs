use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Validation Error: {0}")]
    ValidationError(String),
}
