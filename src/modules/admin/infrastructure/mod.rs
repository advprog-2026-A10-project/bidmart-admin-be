use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;

pub mod controllers;
pub mod repositories;
pub mod services;

pub fn create_router() -> Router<crate::modules::auth::infrastructure::AppState> {
    Router::new()
        .route(
            "/admin",
            get(controllers::get_dashboard_summary).options(cors_preflight),
        )
        .route(
            "/admin/dashboard",
            get(controllers::get_dashboard_summary).options(cors_preflight),
        )
        .route(
            "/admin/users",
            get(controllers::list_users).options(cors_preflight),
        )
        .route(
            "/admin/users/:user_id",
            get(controllers::get_user_detail).options(cors_preflight),
        )
        .route(
            "/admin/users/:user_id/sessions",
            get(controllers::list_user_sessions).options(cors_preflight),
        )
        .route(
            "/admin/moderation/listings",
            get(controllers::list_moderation_listings).options(cors_preflight),
        )
        .route(
            "/admin/moderation/listings/:listing_id",
            get(controllers::get_moderation_listing).options(cors_preflight),
        )
        .route(
            "/admin/disputes",
            get(controllers::list_disputes).options(cors_preflight),
        )
        .route(
            "/admin/disputes/:dispute_id",
            get(controllers::get_dispute_detail).options(cors_preflight),
        )
        .route(
            "/admin/disputes/:dispute_id/resolve",
            post(controllers::resolve_dispute).options(cors_preflight),
        )
}

async fn cors_preflight() -> StatusCode {
    StatusCode::NO_CONTENT
}
