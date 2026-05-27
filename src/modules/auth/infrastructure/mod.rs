use axum::http::{header, HeaderName, HeaderValue, Method, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use serde_json::json;
use sqlx::postgres::PgPool;
use tower_http::cors::CorsLayer;

pub mod controllers;
pub mod middleware;
pub mod repositories;

#[derive(Clone)]
pub struct AppState {
    pub auth_pool: PgPool,
    pub core_pool: PgPool,
    pub auth_base_url: String,
    pub http_timeout_ms: u64,
    pub authz_cache_ttl_seconds: i64,
    pub authz_cache: middleware::AuthzCache,
    pub session_cookie_name: String,
    pub session_cookie_secure: bool,
    pub session_cookie_same_site: String,
    pub session_cookie_max_age_seconds: i64,
}

pub fn create_router_with_cors_origins(state: AppState, allowed_origins: &[String]) -> Router {
    let allowed_origins = allowed_origins
        .iter()
        .map(|origin| {
            HeaderValue::try_from(origin.as_str())
                .expect("CORS origins are validated during configuration loading")
        })
        .collect::<Vec<_>>();

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            header::ACCEPT,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            HeaderName::from_static("x-request-id"),
        ]);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(ready_check))
        .route(
            "/auth/login",
            post(controllers::login).options(cors_preflight),
        )
        .route(
            "/admin/auth/login",
            post(controllers::login).options(cors_preflight),
        )
        .route(
            "/auth/logout",
            post(controllers::logout).options(cors_preflight),
        )
        .route(
            "/admin/auth/logout",
            post(controllers::logout).options(cors_preflight),
        )
        .route(
            "/auth/validate",
            post(controllers::validate_session).options(cors_preflight),
        )
        .route(
            "/admin/auth/me",
            get(controllers::me).options(cors_preflight),
        )
        .merge(crate::modules::admin::infrastructure::create_router());
    app.with_state(state).layer(cors)
}

async fn health_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(json!({"status": "ok", "service": "bidmart-admin-be"}))
}

async fn ready_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(json!({"ready": true}))
}

async fn cors_preflight() -> StatusCode {
    StatusCode::NO_CONTENT
}
