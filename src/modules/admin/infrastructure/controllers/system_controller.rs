use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;

use crate::modules::admin::application::dto::{
    SecurityRuntimeConfigDto, SystemActivitySnapshotDto, SystemSecuritySnapshotDto,
    UpdateSecurityPolicyCommand,
};
use crate::modules::admin::application::use_cases::{SystemActivityUseCase, SystemSecurityUseCase};
use crate::modules::admin::infrastructure::controllers::error::{map_authz_error, ApiError};
use crate::modules::admin::infrastructure::repositories::{
    SqlxAuthAdminRepository, SqlxCoreAdminRepository,
};
use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::AppState;

pub async fn get_system_activity(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<SystemActivitySnapshotDto>, ApiError> {
    middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = SystemActivityUseCase::new(repository);
    let snapshot = use_case
        .get_snapshot()
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(snapshot))
}

pub async fn get_system_security(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<SystemSecuritySnapshotDto>, ApiError> {
    middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = SystemSecurityUseCase::new(repository);
    let snapshot = use_case
        .get_snapshot()
        .await
        .map_err(ApiError::from_domain)?;

    let runtime = runtime_config_from_state(&state);
    Ok(Json(use_case.compose_snapshot_dto(snapshot, runtime)))
}

pub async fn update_system_security_policy(
    State(state): State<AppState>,
    headers: HeaderMap,
    payload: Result<Json<UpdateSecurityPolicyCommand>, JsonRejection>,
) -> Result<Json<SystemSecuritySnapshotDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = SystemSecurityUseCase::new(repository);
    let snapshot = use_case
        .update_policy(
            command.max_concurrent_sessions,
            &command.enforcement_mode,
            command.force_mfa_for_admin,
        )
        .await
        .map_err(ApiError::from_domain)?;

    let runtime = runtime_config_from_state(&state);
    Ok(Json(use_case.compose_snapshot_dto(snapshot, runtime)))
}

fn runtime_config_from_state(state: &AppState) -> SecurityRuntimeConfigDto {
    SecurityRuntimeConfigDto {
        session_cookie_name: state.session_cookie_name.clone(),
        session_cookie_secure: state.session_cookie_secure,
        session_cookie_same_site: state.session_cookie_same_site.clone(),
        authz_cache_ttl_seconds: state.authz_cache_ttl_seconds,
    }
}
