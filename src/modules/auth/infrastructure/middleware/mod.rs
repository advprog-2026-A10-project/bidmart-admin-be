use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPool;
use tokio::sync::RwLock;

use crate::modules::auth::domain::errors::AuthError;
use crate::modules::auth::domain::traits::AuthGatewayPort;
use crate::modules::auth::infrastructure::repositories::AuthGatewayRepository;
use crate::modules::auth::infrastructure::AppState;

#[derive(Debug, Clone)]
pub struct AdminAuthContext {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub mfa_satisfied: bool,
    pub session_expiry: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CachedAuthzSnapshot {
    pub context: AdminAuthContext,
    pub expires_at: DateTime<Utc>,
}

pub type AuthzCache = Arc<RwLock<HashMap<String, CachedAuthzSnapshot>>>;

#[derive(Debug, Serialize)]
struct ErrorEnvelope {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<BTreeMap<String, Vec<String>>>,
}

#[derive(Debug)]
pub enum AuthzError {
    Message { status: StatusCode, message: String },
}

impl AuthzError {
    fn unauthorized() -> Self {
        Self::Message {
            status: StatusCode::UNAUTHORIZED,
            message: "Unauthorized.".to_string(),
        }
    }
}

impl IntoResponse for AuthzError {
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

pub async fn require_permission(
    state: &AppState,
    headers: &HeaderMap,
    required_permission: &str,
) -> Result<AdminAuthContext, AuthzError> {
    let token = extract_token_from_headers(headers, &state.session_cookie_name)
        .ok_or_else(AuthzError::unauthorized)?;
    let cache_key = token_cache_key(&token);
    let now = Utc::now();

    if let Some(snapshot) = {
        let cache = state.authz_cache.read().await;
        cache.get(&cache_key).cloned()
    } {
        if snapshot.expires_at > now {
            ensure_policy(state, &snapshot.context, required_permission).await?;
            return Ok(snapshot.context);
        }
    }

    let gateway = AuthGatewayRepository::new(state.auth_base_url.clone(), state.http_timeout_ms);
    let validated = gateway.validate(&token).await.map_err(map_auth_error)?;

    let context = AdminAuthContext {
        user_id: validated.user_id,
        name: validated.name,
        email: validated.email,
        email_verified: validated.email_verified,
        mfa_satisfied: validated.mfa_satisfied,
        session_expiry: validated.session_expiry,
        roles: validated.roles,
        permissions: validated.permissions,
    };
    ensure_policy(state, &context, required_permission).await?;

    let expires_at = parse_expiry(&context.session_expiry)
        .unwrap_or_else(|| now + Duration::seconds(state.authz_cache_ttl_seconds.max(5)));
    {
        let mut cache = state.authz_cache.write().await;
        cache.insert(
            cache_key,
            CachedAuthzSnapshot {
                context: context.clone(),
                expires_at,
            },
        );
    }

    Ok(context)
}

pub fn extract_token_from_headers(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    if let Some(token) = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
    {
        return Some(token);
    }

    let cookie_header = headers.get(axum::http::header::COOKIE)?.to_str().ok()?;
    cookie_header
        .split(';')
        .map(str::trim)
        .filter_map(|cookie| cookie.split_once('='))
        .find_map(|(key, value)| {
            if key == cookie_name {
                let token = value.trim();
                if token.is_empty() {
                    None
                } else {
                    Some(token.to_string())
                }
            } else {
                None
            }
        })
}

pub async fn invalidate_cache_for_user(cache: &AuthzCache, user_id: uuid::Uuid) -> usize {
    let mut guard = cache.write().await;
    let before = guard.len();
    guard.retain(|_, snapshot| snapshot.context.user_id != user_id);
    before.saturating_sub(guard.len())
}

pub async fn invalidate_cache_for_role(cache: &AuthzCache, role_name: &str) -> usize {
    let mut guard = cache.write().await;
    let before = guard.len();
    guard.retain(|_, snapshot| {
        !snapshot
            .context
            .roles
            .iter()
            .any(|role| role.eq_ignore_ascii_case(role_name))
    });
    before.saturating_sub(guard.len())
}

async fn ensure_policy(
    state: &AppState,
    context: &AdminAuthContext,
    required_permission: &str,
) -> Result<(), AuthzError> {
    let has_admin_role = context
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("ADMIN"));
    if !has_admin_role {
        return Err(AuthzError::Message {
            status: StatusCode::FORBIDDEN,
            message: "Forbidden. Admin role is required.".to_string(),
        });
    }

    let force_mfa_for_admin = resolve_force_mfa_policy(&state.auth_pool).await;
    if force_mfa_for_admin && !context.mfa_satisfied {
        return Err(AuthzError::Message {
            status: StatusCode::FORBIDDEN,
            message: "Forbidden. MFA must be satisfied.".to_string(),
        });
    }

    let has_permission = context
        .permissions
        .iter()
        .any(|permission| permission == required_permission);
    if !has_permission {
        return Err(AuthzError::Message {
            status: StatusCode::FORBIDDEN,
            message: format!("Forbidden. Missing required permission `{required_permission}`."),
        });
    }

    Ok(())
}

async fn resolve_force_mfa_policy(pool: &PgPool) -> bool {
    match sqlx::query_scalar::<_, bool>(
        r#"
        SELECT force_mfa_for_admin
        FROM admin_security_policy
        WHERE id = 1
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await
    {
        Ok(Some(value)) => value,
        Ok(None) => true,
        Err(error) => {
            tracing::warn!(
                ?error,
                "failed to read admin_security_policy, defaulting force_mfa_for_admin=true"
            );
            true
        }
    }
}

fn parse_expiry(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
}

fn token_cache_key(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn map_auth_error(error: AuthError) -> AuthzError {
    match error {
        AuthError::UpstreamClient { status, message } => AuthzError::Message { status, message },
        AuthError::Dependency(message) => AuthzError::Message {
            status: StatusCode::BAD_GATEWAY,
            message,
        },
        AuthError::Unauthorized(message) => AuthzError::Message {
            status: StatusCode::UNAUTHORIZED,
            message,
        },
        AuthError::Forbidden(message) => AuthzError::Message {
            status: StatusCode::FORBIDDEN,
            message,
        },
    }
}
