use std::collections::BTreeMap;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::modules::admin::domain::errors::AdminError;
use crate::modules::auth::infrastructure::middleware;

#[derive(Debug, serde::Serialize)]
struct ErrorEnvelope {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<BTreeMap<String, Vec<String>>>,
}

pub enum ApiError {
    Message { status: StatusCode, message: String },
}

impl ApiError {
    pub fn from_domain(error: AdminError) -> Self {
        match error {
            AdminError::InvalidInput(message) => Self::Message {
                status: StatusCode::BAD_REQUEST,
                message,
            },
            AdminError::NotFound(message) => Self::Message {
                status: StatusCode::NOT_FOUND,
                message,
            },
            AdminError::Conflict(message) => Self::Message {
                status: StatusCode::CONFLICT,
                message,
            },
            AdminError::Repository(message) => Self::Message {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message,
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Message { status, message } => (
                status,
                Json(ErrorEnvelope {
                    message,
                    errors: None,
                }),
            )
                .into_response(),
        }
    }
}

pub fn map_authz_error(error: middleware::AuthzError) -> ApiError {
    match error {
        middleware::AuthzError::Message { status, message } => {
            ApiError::Message { status, message }
        }
    }
}
