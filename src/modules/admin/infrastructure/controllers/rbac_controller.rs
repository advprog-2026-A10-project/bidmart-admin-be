use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use uuid::Uuid;

use crate::modules::admin::application::dto::{
    CreateRoleCommand, RbacMutationResultDto, RbacPermissionsPanelDto, RbacRoleDetailDto,
    RbacRoleDto, RolePermissionMutationCommand, UserRoleMutationCommand,
};
use crate::modules::admin::application::use_cases::RbacUseCase;
use crate::modules::admin::infrastructure::controllers::error::{map_authz_error, ApiError};
use crate::modules::admin::infrastructure::repositories::SqlxAuthAdminRepository;
use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::AppState;

pub async fn list_roles(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<RbacRoleDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = RbacUseCase::new(repository);
    let roles = use_case.list_roles().await.map_err(ApiError::from_domain)?;

    Ok(Json(roles))
}

pub async fn create_role(
    State(state): State<AppState>,
    headers: HeaderMap,
    payload: Result<Json<CreateRoleCommand>, JsonRejection>,
) -> Result<Json<RbacRoleDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = RbacUseCase::new(repository);
    let role = use_case
        .create_role(&command.name)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(role))
}

pub async fn get_role_detail(
    State(state): State<AppState>,
    Path(role_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<RbacRoleDetailDto>, ApiError> {
    middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(map_authz_error)?;

    let role_id = role_id.parse::<i32>().map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid role id.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = RbacUseCase::new(repository);
    let role = use_case
        .get_role_detail(role_id)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(role))
}

pub async fn get_permissions_panel(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<RbacPermissionsPanelDto>, ApiError> {
    middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = RbacUseCase::new(repository);
    let panel = use_case
        .get_permissions_panel()
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(panel))
}

pub async fn assign_user_role(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
    payload: Result<Json<UserRoleMutationCommand>, JsonRejection>,
) -> Result<Json<RbacMutationResultDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;
    let user_id = Uuid::parse_str(&user_id).map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid user id.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = RbacUseCase::new(repository);
    let result = use_case
        .assign_user_role(user_id, &command.role)
        .await
        .map_err(ApiError::from_domain)?;

    if result.changed {
        let _ = middleware::invalidate_cache_for_user(&state.authz_cache, user_id).await;
    }

    Ok(Json(result))
}

pub async fn revoke_user_role(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
    payload: Result<Json<UserRoleMutationCommand>, JsonRejection>,
) -> Result<Json<RbacMutationResultDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;
    let user_id = Uuid::parse_str(&user_id).map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid user id.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = RbacUseCase::new(repository);
    let result = use_case
        .revoke_user_role(user_id, &command.role)
        .await
        .map_err(ApiError::from_domain)?;

    if result.changed {
        let _ = middleware::invalidate_cache_for_user(&state.authz_cache, user_id).await;
    }

    Ok(Json(result))
}

pub async fn assign_role_permission(
    State(state): State<AppState>,
    Path(role_name): Path<String>,
    headers: HeaderMap,
    payload: Result<Json<RolePermissionMutationCommand>, JsonRejection>,
) -> Result<Json<RbacMutationResultDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = RbacUseCase::new(repository);
    let result = use_case
        .assign_role_permission(&role_name, &command.permission)
        .await
        .map_err(ApiError::from_domain)?;

    if result.changed {
        let _ = middleware::invalidate_cache_for_role(&state.authz_cache, &role_name).await;
    }

    Ok(Json(result))
}

pub async fn revoke_role_permission(
    State(state): State<AppState>,
    Path(role_name): Path<String>,
    headers: HeaderMap,
    payload: Result<Json<RolePermissionMutationCommand>, JsonRejection>,
) -> Result<Json<RbacMutationResultDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = RbacUseCase::new(repository);
    let result = use_case
        .revoke_role_permission(&role_name, &command.permission)
        .await
        .map_err(ApiError::from_domain)?;

    if result.changed {
        let _ = middleware::invalidate_cache_for_role(&state.authz_cache, &role_name).await;
    }

    Ok(Json(result))
}
