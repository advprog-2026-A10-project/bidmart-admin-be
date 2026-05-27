use std::collections::BTreeMap;

use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::auth::infrastructure::AppState;

use super::middleware;
use super::services;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginCommand {
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDto {
    pub user: services::AuthServiceUser,
    pub access_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateResponseDto {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub mfa_satisfied: bool,
    pub session_expiry: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminMeResponseDto {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub mfa_satisfied: bool,
    pub session_expiry: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponseDto {
    pub message: String,
}

#[derive(Debug, Serialize)]
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
    fn from_json_rejection(rejection: JsonRejection) -> Self {
        Self::Message {
            status: rejection.status(),
            message: "Invalid JSON payload.".to_string(),
        }
    }

    fn from_validation_errors(errors: validator::ValidationErrors) -> Self {
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

    fn from_forward_error(error: services::ForwardError) -> Self {
        match error {
            services::ForwardError::Client(status, message) => Self::Message { status, message },
            services::ForwardError::Dependency(message) => Self::Message {
                status: StatusCode::BAD_GATEWAY,
                message,
            },
        }
    }

    fn unauthorized() -> Self {
        Self::Message {
            status: StatusCode::UNAUTHORIZED,
            message: "Unauthorized.".to_string(),
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

pub async fn login(
    State(state): State<AppState>,
    payload: Result<Json<LoginCommand>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(command) = payload.map_err(ApiError::from_json_rejection)?;
    command
        .validate()
        .map_err(ApiError::from_validation_errors)?;

    let auth_response = services::login_with_auth_service(
        &state.auth_base_url,
        state.http_timeout_ms,
        &command.email,
        &command.password,
    )
    .await
    .map_err(ApiError::from_forward_error)?;

    if auth_response.requires_mfa {
        return Err(ApiError::Message {
            status: StatusCode::FORBIDDEN,
            message: "Admin login with MFA is not yet supported in admin portal.".to_string(),
        });
    }

    let access_token = auth_response
        .access_token
        .ok_or_else(ApiError::unauthorized)?;
    let session_token = access_token.clone();
    let user = auth_response.user.ok_or_else(ApiError::unauthorized)?;

    let validated =
        services::validate_with_token(&state.auth_base_url, state.http_timeout_ms, &access_token)
            .await
            .map_err(ApiError::from_forward_error)?;

    let has_admin_role = validated
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("ADMIN"));

    if !has_admin_role {
        let _ =
            services::logout_with_token(&state.auth_base_url, state.http_timeout_ms, &access_token)
                .await;
        return Err(ApiError::Message {
            status: StatusCode::FORBIDDEN,
            message: "Forbidden. Admin role is required.".to_string(),
        });
    }

    let payload = Json(LoginResponseDto { user, access_token });
    let mut response = payload.into_response();
    append_session_cookie(
        &mut response,
        &state.session_cookie_name,
        &session_token,
        state.session_cookie_max_age_seconds,
        &state.session_cookie_same_site,
        state.session_cookie_secure,
    );
    Ok(response)
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    if let Some(token) = extract_token_from_headers(&headers, &state.session_cookie_name) {
        let _ =
            services::logout_with_token(&state.auth_base_url, state.http_timeout_ms, &token).await;
    }

    let payload = Json(MessageResponseDto {
        message: "Logout successful.".to_string(),
    });
    let mut response = payload.into_response();
    append_clear_session_cookie(
        &mut response,
        &state.session_cookie_name,
        &state.session_cookie_same_site,
        state.session_cookie_secure,
    );
    Ok(response)
}

pub async fn validate_session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ValidateResponseDto>, ApiError> {
    let auth = middleware::require_permission(&state, &headers, "admin:access")
        .await
        .map_err(|error| match error {
            middleware::AuthzError::Message { status, message } => {
                ApiError::Message { status, message }
            }
        })?;

    Ok(Json(ValidateResponseDto {
        user_id: auth.user_id,
        name: auth.name,
        email: auth.email,
        email_verified: auth.email_verified,
        mfa_satisfied: auth.mfa_satisfied,
        session_expiry: auth.session_expiry,
        is_admin: true,
    }))
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<AdminMeResponseDto>, ApiError> {
    let auth = middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(|error| match error {
            middleware::AuthzError::Message { status, message } => {
                ApiError::Message { status, message }
            }
        })?;

    Ok(Json(AdminMeResponseDto {
        user_id: auth.user_id,
        name: auth.name,
        email: auth.email,
        email_verified: auth.email_verified,
        mfa_satisfied: auth.mfa_satisfied,
        session_expiry: auth.session_expiry,
        roles: auth.roles,
        permissions: auth.permissions,
    }))
}

fn extract_token_from_headers(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    middleware::extract_token_from_headers(headers, cookie_name)
}

fn append_session_cookie(
    response: &mut Response,
    cookie_name: &str,
    token: &str,
    max_age_seconds: i64,
    same_site: &str,
    secure: bool,
) {
    let mut cookie = format!(
        "{cookie_name}={token}; Path=/; Max-Age={max_age_seconds}; HttpOnly; SameSite={same_site}"
    );
    if secure {
        cookie.push_str("; Secure");
    }
    if let Ok(value) = axum::http::HeaderValue::from_str(&cookie) {
        response
            .headers_mut()
            .insert(axum::http::header::SET_COOKIE, value);
    }
}

fn append_clear_session_cookie(
    response: &mut Response,
    cookie_name: &str,
    same_site: &str,
    secure: bool,
) {
    let mut cookie = format!("{cookie_name}=; Path=/; Max-Age=0; HttpOnly; SameSite={same_site}");
    if secure {
        cookie.push_str("; Secure");
    }
    if let Ok(value) = axum::http::HeaderValue::from_str(&cookie) {
        response
            .headers_mut()
            .insert(axum::http::header::SET_COOKIE, value);
    }
}
