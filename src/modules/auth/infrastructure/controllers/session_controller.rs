use std::sync::Arc;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::modules::auth::application::dto::{
    AdminMeResponseDto, MessageResponseDto, ValidateResponseDto,
};
use crate::modules::auth::application::use_cases::{
    to_admin_me_response, to_validate_response, LogoutUseCase,
};
use crate::modules::auth::domain::entities::AuthValidation;
use crate::modules::auth::infrastructure::controllers::error::ApiError;
use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::repositories::AuthGatewayRepository;
use crate::modules::auth::infrastructure::AppState;

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    if let Some(token) =
        middleware::extract_token_from_headers(&headers, &state.session_cookie_name)
    {
        let gateway = Arc::new(AuthGatewayRepository::new(
            state.auth_base_url.clone(),
            state.http_timeout_ms,
        ));
        let use_case = LogoutUseCase::new(gateway);
        use_case.execute(&token).await;
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
        .map_err(ApiError::from_authz_error)?;

    let validation = AuthValidation {
        user_id: auth.user_id,
        name: auth.name,
        email: auth.email,
        email_verified: auth.email_verified,
        mfa_satisfied: auth.mfa_satisfied,
        session_expiry: auth.session_expiry,
        roles: auth.roles,
        permissions: auth.permissions,
    };

    Ok(Json(to_validate_response(&validation)))
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<AdminMeResponseDto>, ApiError> {
    let auth = middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(ApiError::from_authz_error)?;

    let validation = AuthValidation {
        user_id: auth.user_id,
        name: auth.name,
        email: auth.email,
        email_verified: auth.email_verified,
        mfa_satisfied: auth.mfa_satisfied,
        session_expiry: auth.session_expiry,
        roles: auth.roles,
        permissions: auth.permissions,
    };

    Ok(Json(to_admin_me_response(&validation)))
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
