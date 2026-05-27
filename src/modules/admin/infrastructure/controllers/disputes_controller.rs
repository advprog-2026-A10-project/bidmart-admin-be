use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use uuid::Uuid;

use crate::modules::admin::application::dto::{DisputeDto, ResolveDisputeCommand};
use crate::modules::admin::application::use_cases::DisputeUseCase;
use crate::modules::admin::domain::entities::DisputeResolutionOutcome;
use crate::modules::admin::infrastructure::controllers::error::{map_authz_error, ApiError};
use crate::modules::admin::infrastructure::repositories::SqlxCoreAdminRepository;
use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::AppState;

pub async fn list_disputes(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<DisputeDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "order:intervene")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = DisputeUseCase::new(repository);
    let disputes = use_case
        .list_disputes()
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(disputes))
}

pub async fn get_dispute_detail(
    State(state): State<AppState>,
    Path(dispute_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<DisputeDto>, ApiError> {
    middleware::require_permission(&state, &headers, "order:intervene")
        .await
        .map_err(map_authz_error)?;

    let dispute_id = Uuid::parse_str(&dispute_id).map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid dispute id.".to_string(),
    })?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = DisputeUseCase::new(repository);
    let dispute = use_case
        .get_dispute(dispute_id)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(dispute))
}

pub async fn resolve_dispute(
    State(state): State<AppState>,
    Path(dispute_id): Path<String>,
    headers: HeaderMap,
    payload: Result<Json<ResolveDisputeCommand>, JsonRejection>,
) -> Result<Json<DisputeDto>, ApiError> {
    middleware::require_permission(&state, &headers, "order:intervene")
        .await
        .map_err(map_authz_error)?;

    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;

    let dispute_id = Uuid::parse_str(&dispute_id).map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid dispute id.".to_string(),
    })?;

    let outcome = DisputeResolutionOutcome::parse(&command.outcome).ok_or(ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid outcome. Use BUYER or SELLER.".to_string(),
    })?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = DisputeUseCase::new(repository);
    let dispute = use_case
        .resolve_dispute(dispute_id, outcome, &command.resolution)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(dispute))
}
