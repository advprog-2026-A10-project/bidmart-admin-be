use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;

use crate::modules::admin::application::dto::{
    CategoryDto, CreateCategoryCommand, UpdateCategoryCommand,
};
use crate::modules::admin::application::use_cases::CategoryUseCase;
use crate::modules::admin::infrastructure::controllers::error::{map_authz_error, ApiError};
use crate::modules::admin::infrastructure::repositories::SqlxCoreAdminRepository;
use crate::modules::auth::infrastructure::middleware;
use crate::modules::auth::infrastructure::AppState;

pub async fn list_categories(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<CategoryDto>>, ApiError> {
    middleware::require_permission(&state, &headers, "category:manage")
        .await
        .map_err(map_authz_error)?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = CategoryUseCase::new(repository);
    let categories = use_case
        .list_categories()
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(categories))
}

pub async fn create_category(
    State(state): State<AppState>,
    headers: HeaderMap,
    payload: Result<Json<CreateCategoryCommand>, JsonRejection>,
) -> Result<Json<CategoryDto>, ApiError> {
    middleware::require_permission(&state, &headers, "category:manage")
        .await
        .map_err(map_authz_error)?;

    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = CategoryUseCase::new(repository);
    let category = use_case
        .create_category(
            &command.name,
            &command.slug,
            command.parent_id,
            command.image_url,
        )
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(category))
}

pub async fn update_category(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    headers: HeaderMap,
    payload: Result<Json<UpdateCategoryCommand>, JsonRejection>,
) -> Result<Json<CategoryDto>, ApiError> {
    middleware::require_permission(&state, &headers, "category:manage")
        .await
        .map_err(map_authz_error)?;

    let category_id = category_id.parse::<i32>().map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid category id.".to_string(),
    })?;
    let Json(command) = payload.map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload.".to_string(),
    })?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = CategoryUseCase::new(repository);
    let category = use_case
        .update_category(
            category_id,
            &command.name,
            &command.slug,
            command.parent_id,
            command.image_url,
        )
        .await
        .map_err(ApiError::from_domain)?;

    Ok(Json(category))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    middleware::require_permission(&state, &headers, "category:manage")
        .await
        .map_err(map_authz_error)?;

    let category_id = category_id.parse::<i32>().map_err(|_| ApiError::Message {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid category id.".to_string(),
    })?;

    let repository = Arc::new(SqlxCoreAdminRepository::new(state.core_pool.clone()));
    let use_case = CategoryUseCase::new(repository);
    use_case
        .delete_category(category_id)
        .await
        .map_err(ApiError::from_domain)?;

    Ok(StatusCode::NO_CONTENT)
}
