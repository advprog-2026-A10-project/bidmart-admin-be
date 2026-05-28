use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use validator::Validate;

use crate::modules::auth::application::dto::{LoginCommand, LoginResponseDto};
use crate::modules::auth::application::use_cases::LoginUseCase;
use crate::modules::auth::infrastructure::controllers::error::ApiError;
use crate::modules::auth::infrastructure::repositories::AuthGatewayRepository;
use crate::modules::auth::infrastructure::AppState;

pub async fn login(
    State(state): State<AppState>,
    payload: Result<Json<LoginCommand>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(command) = payload.map_err(ApiError::from_json_rejection)?;
    command
        .validate()
        .map_err(ApiError::from_validation_errors)?;

    let gateway = Arc::new(AuthGatewayRepository::new(
        state.auth_base_url.clone(),
        state.http_timeout_ms,
    ));
    let use_case = LoginUseCase::new(gateway);
    let outcome = use_case
        .execute(&command.email, &command.password)
        .await
        .map_err(ApiError::from_auth_error)?;

    let payload = Json(LoginResponseDto {
        user: outcome.user,
        access_token: outcome.access_token.clone(),
    });
    let mut response = payload.into_response();
    append_session_cookie(
        &mut response,
        &state.session_cookie_name,
        &outcome.access_token,
        state.session_cookie_max_age_seconds,
        &state.session_cookie_same_site,
        state.session_cookie_secure,
    );

    Ok(response)
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
