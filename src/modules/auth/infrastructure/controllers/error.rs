use std::collections::BTreeMap;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::modules::auth::domain::errors::AuthError;

#[derive(Debug, serde::Serialize)]
struct ErrorEnvelope {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<BTreeMap<String, Vec<String>>>,
}

pub enum ApiError {
    Validation {
        message: String,
        errors: BTreeMap<String, Vec<String>>,
    },
    Message {
        status: StatusCode,
        message: String,
    },
}

impl ApiError {
    pub fn from_json_rejection(rejection: JsonRejection) -> Self {
        Self::Message {
            status: rejection.status(),
            message: "Invalid JSON payload.".to_string(),
        }
    }

    pub fn from_validation_errors(errors: validator::ValidationErrors) -> Self {
        let mut field_errors = BTreeMap::new();
        for (field, errors_for_field) in errors.field_errors() {
            let messages = errors_for_field
                .iter()
                .map(|error| {
                    error
                        .message
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "Invalid value.".to_string())
                })
                .collect::<Vec<_>>();
            field_errors.insert(field.to_string(), messages);
        }
        Self::Validation {
            message: "Validation error".to_string(),
            errors: field_errors,
        }
    }

    pub fn from_auth_error(error: AuthError) -> Self {
        match error {
            AuthError::UpstreamClient { status, message } => Self::Message { status, message },
            AuthError::Dependency(message) => Self::Message {
                status: StatusCode::BAD_GATEWAY,
                message,
            },
            AuthError::Unauthorized(message) => Self::Message {
                status: StatusCode::UNAUTHORIZED,
                message,
            },
            AuthError::Forbidden(message) => Self::Message {
                status: StatusCode::FORBIDDEN,
                message,
            },
        }
    }

    pub fn from_authz_error(
        error: crate::modules::auth::infrastructure::middleware::AuthzError,
    ) -> Self {
        match error {
            crate::modules::auth::infrastructure::middleware::AuthzError::Message {
                status,
                message,
            } => Self::Message { status, message },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Validation { message, errors } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorEnvelope {
                    message,
                    errors: Some(errors),
                }),
            )
                .into_response(),
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
