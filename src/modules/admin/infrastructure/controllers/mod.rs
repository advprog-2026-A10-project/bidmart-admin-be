use std::collections::BTreeMap;

use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::AppState;

use super::services;

#[derive(Debug, Serialize)]
struct ErrorEnvelope {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<BTreeMap<String, Vec<String>>>,
}

pub enum ApiError {
    Message { status: StatusCode, message: String },
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveDisputeCommand {
    pub outcome: String,
    pub resolution: String,
}

pub async fn get_dashboard_summary(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<services::AdminDashboardSummaryDto>, ApiError> {
    middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(map_authz_error)?;

    let summary = services::get_dashboard_summary(&state.auth_pool)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to load dashboard summary.".to_string(),
        })?;

    Ok(Json(summary))
}

pub async fn list_users(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<services::ManagedUserDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let users = services::list_managed_users(&state.auth_pool)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to load users.".to_string(),
        })?;

    Ok(Json(users))
}

pub async fn get_user_detail(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<services::ManagedUserDto>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid user id.".to_string(),
    })?;

    let user = services::get_managed_user(&state.auth_pool, user_id)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to load user detail.".to_string(),
        })?
        .ok_or_else(|| ApiError::Message {
            status: StatusCode::NOT_FOUND,
            message: "User not found.".to_string(),
        })?;

    Ok(Json(user))
}

pub async fn list_user_sessions(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Vec<services::ManagedUserSessionDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "user:suspend")
        .await
        .map_err(map_authz_error)?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid user id.".to_string(),
    })?;

    let user_exists = services::get_managed_user(&state.auth_pool, user_id)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to verify user.".to_string(),
        })?
        .is_some();

    if !user_exists {
        return Err(ApiError::Message {
            status: StatusCode::NOT_FOUND,
            message: "User not found.".to_string(),
        });
    }

    let sessions = services::list_user_sessions(&state.auth_pool, user_id)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to load user sessions.".to_string(),
        })?;

    Ok(Json(sessions))
}

pub async fn list_moderation_listings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<services::ModerationListingDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "listing:moderate")
        .await
        .map_err(map_authz_error)?;

    let listings = services::list_moderation_listings(&state.core_pool)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to load listings.".to_string(),
        })?;

    Ok(Json(listings))
}

pub async fn get_moderation_listing(
    State(state): State<AppState>,
    Path(listing_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<services::ModerationListingDto>, ApiError> {
    middleware::require_permission(&state, &headers, "listing:moderate")
        .await
        .map_err(map_authz_error)?;

    let listing_id = Uuid::parse_str(&listing_id).map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid listing id.".to_string(),
    })?;

    let listing = services::get_moderation_listing(&state.core_pool, listing_id)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to load listing detail.".to_string(),
        })?
        .ok_or_else(|| ApiError::Message {
            status: StatusCode::NOT_FOUND,
            message: "Listing not found.".to_string(),
        })?;

    Ok(Json(listing))
}

pub async fn list_disputes(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<services::DisputeDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "order:intervene")
        .await
        .map_err(map_authz_error)?;

    let disputes = services::list_disputes(&state.core_pool)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to load disputes.".to_string(),
        })?;

    Ok(Json(disputes))
}

pub async fn get_dispute_detail(
    State(state): State<AppState>,
    Path(dispute_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<services::DisputeDto>, ApiError> {
    middleware::require_permission(&state, &headers, "order:intervene")
        .await
        .map_err(map_authz_error)?;

    let dispute_id = Uuid::parse_str(&dispute_id).map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid dispute id.".to_string(),
    })?;

    let dispute = services::get_dispute(&state.core_pool, dispute_id)
        .await
        .map_err(|_| ApiError::Message {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to load dispute detail.".to_string(),
        })?
        .ok_or_else(|| ApiError::Message {
            status: StatusCode::NOT_FOUND,
            message: "Dispute not found.".to_string(),
        })?;

    Ok(Json(dispute))
}

pub async fn resolve_dispute(
    State(state): State<AppState>,
    Path(dispute_id): Path<String>,
    headers: HeaderMap,
    payload: Result<Json<ResolveDisputeCommand>, JsonRejection>,
) -> Result<Json<services::DisputeDto>, ApiError> {
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

    let outcome =
        services::DisputeResolutionOutcome::parse(&command.outcome).ok_or(ApiError::Message {
            status: StatusCode::BAD_REQUEST,
            message: "Invalid outcome. Use BUYER or SELLER.".to_string(),
        })?;

    let resolution = command.resolution.trim();
    if resolution.is_empty() {
        return Err(ApiError::Message {
            status: StatusCode::BAD_REQUEST,
            message: "Resolution is required.".to_string(),
        });
    }

    let dispute = services::resolve_dispute(&state.core_pool, dispute_id, outcome, resolution)
        .await
        .map_err(|error| match error {
            services::ResolveDisputeError::NotFound => ApiError::Message {
                status: StatusCode::NOT_FOUND,
                message: "Dispute not found.".to_string(),
            },
            services::ResolveDisputeError::AlreadyResolved => ApiError::Message {
                status: StatusCode::CONFLICT,
                message: "Dispute already resolved.".to_string(),
            },
            services::ResolveDisputeError::Database(error) => {
                tracing::error!(?error, "failed to resolve dispute");
                ApiError::Message {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to resolve dispute.".to_string(),
                }
            }
        })?;

    Ok(Json(dispute))
}

fn map_authz_error(error: middleware::AuthzError) -> ApiError {
    match error {
        middleware::AuthzError::Message { status, message } => {
            ApiError::Message { status, message }
        }
    }
}
