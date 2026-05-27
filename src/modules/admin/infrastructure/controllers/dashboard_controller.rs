use std::sync::Arc;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;

use crate::modules::admin::application::dto::AdminDashboardSummaryDto;
use crate::modules::admin::application::use_cases::GetDashboardSummaryUseCase;
use crate::modules::admin::infrastructure::controllers::error::{map_authz_error, ApiError};
use crate::modules::admin::infrastructure::repositories::SqlxAuthAdminRepository;
use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::AppState;

pub async fn get_dashboard_summary(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<AdminDashboardSummaryDto>, ApiError> {
    middleware::require_permission(&state, &headers, "admin:auth:read")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxAuthAdminRepository::new(state.auth_pool.clone()));
    let use_case = GetDashboardSummaryUseCase::new(repository);
    let summary = use_case.execute().await.map_err(ApiError::from_domain)?;

    Ok(Json(summary))
}
