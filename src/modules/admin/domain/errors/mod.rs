use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdminError {
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Repository(String),
}
