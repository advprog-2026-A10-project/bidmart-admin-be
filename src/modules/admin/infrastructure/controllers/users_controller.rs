use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use uuid::Uuid;

use crate::modules::admin::application::dto::{
    ManagedUserDto, ManagedUserSessionDto, SessionActionResultDto,
};
use crate::modules::admin::application::use_cases::UserManagementUseCase;
use crate::modules::admin::infrastructure::controllers::error::{map_authz_error, ApiError};
use crate::modules::admin::infrastructure::repositories::SqlxAuthAdminRepository;
use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::AppState;

pub async fn list_users(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<ManagedUserDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = UserManagementUseCase::new(repository);
    let users = use_case.list_users().await.map_err(ApiError::from_domain)?;

    Ok(Json(users))
}

pub async fn get_user_detail(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ManagedUserDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| ApiError::Message {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: "Invalid user id.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = UserManagementUseCase::new(repository);
    let user = use_case
        .get_user(user_id)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(user))
}

pub async fn list_user_sessions(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Vec<ManagedUserSessionDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| ApiError::Message {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: "Invalid user id.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = UserManagementUseCase::new(repository);
    let sessions = use_case
        .list_user_sessions(user_id)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(sessions))
}

pub async fn revoke_user_session(
    State(state): State<AppState>,
    Path((user_id, session_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<SessionActionResultDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| ApiError::Message {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: "Invalid user id.".to_string(),
    })?;
    let session_id = Uuid::parse_str(&session_id).map_err(|_| ApiError::Message {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: "Invalid session id.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = UserManagementUseCase::new(repository);
    let result = use_case
        .revoke_user_session(user_id, session_id)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(result))
}

pub async fn revoke_all_user_sessions(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<SessionActionResultDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| ApiError::Message {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: "Invalid user id.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = UserManagementUseCase::new(repository);
    let result = use_case
        .revoke_all_user_sessions(user_id)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(result))
}
