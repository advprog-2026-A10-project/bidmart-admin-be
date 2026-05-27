use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("{message}")]
    UpstreamClient { status: StatusCode, message: String },
    #[error("{0}")]
    Dependency(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
}
