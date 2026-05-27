use config::ConfigError;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub auth_database_url: String,
    pub core_database_url: String,
    pub auth_base_url: String,
    pub cors_allowed_origins: Vec<String>,
    pub auth_http_timeout_ms: u64,
    pub admin_session_cookie_name: String,
    pub admin_session_cookie_secure: bool,
    pub admin_session_cookie_same_site: String,
    pub admin_session_cookie_max_age_seconds: i64,
    pub authz_cache_ttl_seconds: i64,
    pub amqp_url: Option<String>,
    pub amqp_exchange: String,
    pub amqp_admin_authz_queue: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let project_root = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(".env");

        dotenv::dotenv().ok();

        if !project_root.exists() {
            let cargo_root = std::env::var("CARGO_MANIFEST_DIR")
                .map(|p| PathBuf::from(p).parent().unwrap().join(".env"))
                .ok();

            if let Some(path) = cargo_root {
                if path.exists() {
                    dotenv::from_path(&path).ok();
                }
            }
        }

        let server_host = required_env("APP_SERVER_HOST")?;
        let server_port = required_env("APP_SERVER_PORT")?
            .parse::<u16>()
            .map_err(|_| ConfigError::Message("Invalid APP_SERVER_PORT".to_string()))?;
        let auth_database_url = required_env("APP_AUTH_DATABASE_URL")?;
        let core_database_url = required_env("APP_CORE_DATABASE_URL")?;
        let auth_base_url = required_env("APP_AUTH_BASE_URL")?;
        let cors_allowed_origins = optional_env(
            "APP_CORS_ALLOWED_ORIGINS",
            "http://localhost:5173,http://127.0.0.1:5173",
        )
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        let auth_http_timeout_ms = optional_parsed_env("APP_AUTH_HTTP_TIMEOUT_MS", 8000_u64)?;
        let admin_session_cookie_name =
            optional_env("APP_ADMIN_SESSION_COOKIE_NAME", "admin_session");
        let admin_session_cookie_secure =
            optional_parsed_env("APP_ADMIN_SESSION_COOKIE_SECURE", false)?;
        let admin_session_cookie_same_site =
            optional_env("APP_ADMIN_SESSION_COOKIE_SAME_SITE", "Lax");
        let admin_session_cookie_max_age_seconds =
            optional_parsed_env("APP_ADMIN_SESSION_COOKIE_MAX_AGE_SECONDS", 86_400_i64)?;
        let authz_cache_ttl_seconds =
            optional_parsed_env("APP_ADMIN_AUTHZ_CACHE_TTL_SECONDS", 60_i64)?;
        let amqp_url = std::env::var("APP_AMQP_URL")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let amqp_exchange = optional_env("APP_AMQP_EXCHANGE", "bidmart.domain.events");
        let amqp_admin_authz_queue =
            optional_env("APP_AMQP_ADMIN_AUTHZ_QUEUE", "admin.authz.cache.invalidate");

        Ok(AppConfig {
            server_host,
            server_port,
            auth_database_url,
            core_database_url,
            auth_base_url,
            cors_allowed_origins,
            auth_http_timeout_ms,
            admin_session_cookie_name,
            admin_session_cookie_secure,
            admin_session_cookie_same_site,
            admin_session_cookie_max_age_seconds,
            authz_cache_ttl_seconds,
            amqp_url,
            amqp_exchange,
            amqp_admin_authz_queue,
        })
    }
}

fn required_env(key: &str) -> Result<String, ConfigError> {
    std::env::var(key)
        .or_else(|_| std::env::var(key.to_ascii_lowercase()))
        .map_err(|_| ConfigError::Message(format!("Missing {key}")))
}

fn optional_env(key: &str, default_value: &str) -> String {
    std::env::var(key)
        .or_else(|_| std::env::var(key.to_ascii_lowercase()))
        .unwrap_or_else(|_| default_value.to_string())
}

fn optional_parsed_env<T>(key: &str, default_value: T) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
{
    match std::env::var(key).or_else(|_| std::env::var(key.to_ascii_lowercase())) {
        Ok(value) => value
            .parse::<T>()
            .map_err(|_| ConfigError::Message(format!("Invalid {key}"))),
        Err(_) => Ok(default_value),
    }
}
