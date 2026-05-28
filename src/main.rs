mod infrastructure;
mod modules;
mod shared;

use axum::serve;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use infrastructure::amqp::spawn_authz_cache_invalidation_consumer;
use infrastructure::config::AppConfig;
use infrastructure::database::{create_auth_pool, create_core_pool};
use infrastructure::logger::init_tracer;
use modules::auth::infrastructure::create_router_with_cors_origins;
use modules::auth::infrastructure::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracer();

    let config = AppConfig::new().expect("Failed to load configuration from .env file");

    let auth_pool = create_auth_pool(&config.auth_database_url).await?;
    let core_pool = create_core_pool(&config.core_database_url).await?;

    let app_state = AppState {
        auth_pool,
        core_pool,
        auth_base_url: config.auth_base_url.clone(),
        http_timeout_ms: config.auth_http_timeout_ms,
        authz_cache_ttl_seconds: config.authz_cache_ttl_seconds,
        authz_cache: Arc::new(RwLock::new(HashMap::new())),
        session_cookie_name: config.admin_session_cookie_name.clone(),
        session_cookie_secure: config.admin_session_cookie_secure,
        session_cookie_same_site: config.admin_session_cookie_same_site.clone(),
        session_cookie_max_age_seconds: config.admin_session_cookie_max_age_seconds,
    };

    spawn_authz_cache_invalidation_consumer(
        app_state.authz_cache.clone(),
        config.amqp_url.clone(),
        config.amqp_exchange.clone(),
        config.amqp_admin_authz_queue.clone(),
    );

    let router = create_router_with_cors_origins(app_state, &config.cors_allowed_origins);

    let address = format!("{}:{}", config.server_host, config.server_port);
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("Starting server on {}", address);

    serve(listener, router).await?;

    Ok(())
}
