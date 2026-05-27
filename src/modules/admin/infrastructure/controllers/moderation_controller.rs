use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use uuid::Uuid;

use crate::modules::admin::application::dto::ModerationListingDto;
use crate::modules::admin::application::use_cases::ModerationUseCase;
use crate::modules::admin::infrastructure::controllers::error::{map_authz_error, ApiError};
use crate::modules::admin::infrastructure::repositories::SqlxCoreAdminRepository;
use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::AppState;

pub async fn list_moderation_listings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<ModerationListingDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "listing:moderate")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = ModerationUseCase::new(repository);
    let listings = use_case
        .list_listings()
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(listings))
}

pub async fn get_moderation_listing(
    State(state): State<AppState>,
    Path(listing_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ModerationListingDto>, ApiError> {
    middleware::require_permission(&state, &headers, "listing:moderate")
        .await
        .map_err(map_authz_error)?;

    let listing_id = Uuid::parse_str(&listing_id).map_err(|_| ApiError::Message {
        status: axum::http::StatusCode::BAD_REQUEST,
        message: "Invalid listing id.".to_string(),
    })?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = ModerationUseCase::new(repository);
    let listing = use_case
        .get_listing(listing_id)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(listing))
}
