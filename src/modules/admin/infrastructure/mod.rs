use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;

pub mod controllers;
pub mod repositories;

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
            "/admin/users/:user_id/sessions/:session_id/revoke",
            post(controllers::revoke_user_session).options(cors_preflight),
        )
        .route(
            "/admin/users/:user_id/sessions/revoke-all",
            post(controllers::revoke_all_user_sessions).options(cors_preflight),
        )
        .route(
            "/admin/rbac/roles",
            get(controllers::list_roles)
                .post(controllers::create_role)
                .options(cors_preflight),
        )
        .route(
            "/admin/rbac/roles/:role_id",
            get(controllers::get_role_detail).options(cors_preflight),
        )
        .route(
            "/admin/rbac/permissions",
            get(controllers::get_permissions_panel).options(cors_preflight),
        )
        .route(
            "/admin/rbac/users/:user_id/roles/assign",
            post(controllers::assign_user_role).options(cors_preflight),
        )
        .route(
            "/admin/rbac/users/:user_id/roles/revoke",
            post(controllers::revoke_user_role).options(cors_preflight),
        )
        .route(
            "/admin/rbac/roles/:role_name/permissions/assign",
            post(controllers::assign_role_permission).options(cors_preflight),
        )
        .route(
            "/admin/rbac/roles/:role_name/permissions/revoke",
            post(controllers::revoke_role_permission).options(cors_preflight),
        )
        .route(
            "/admin/system/activity",
            get(controllers::get_system_activity).options(cors_preflight),
        )
        .route(
            "/admin/system/security",
            get(controllers::get_system_security)
                .post(controllers::update_system_security_policy)
                .options(cors_preflight),
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
