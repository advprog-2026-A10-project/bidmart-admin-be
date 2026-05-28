use async_trait::async_trait;
use axum::http::StatusCode;
use reqwest::Client;
use serde::Deserialize;

use crate::modules::auth::domain::entities::{AuthLoginResult, AuthUser, AuthValidation};
use crate::modules::auth::domain::errors::AuthError;
use crate::modules::auth::domain::traits::AuthGatewayPort;

#[derive(Clone)]
pub struct AuthGatewayRepository {
    auth_base_url: String,
    timeout_ms: u64,
}

impl AuthGatewayRepository {
    pub fn new(auth_base_url: String, timeout_ms: u64) -> Self {
        Self {
            auth_base_url,
            timeout_ms,
        }
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.auth_base_url.trim_end_matches('/'), path)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpstreamLoginResponse {
    requires_mfa: bool,
    #[serde(default)]
    user: Option<AuthUser>,
    #[serde(default)]
    access_token: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpstreamValidateResponse {
    user_id: uuid::Uuid,
    name: String,
    email: String,
    email_verified: bool,
    mfa_satisfied: bool,
    session_expiry: String,
    #[serde(default)]
    roles: Vec<String>,
    #[serde(default)]
    permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ErrorEnvelope {
    message: Option<String>,
}

#[async_trait]
impl AuthGatewayPort for AuthGatewayRepository {
    async fn login(&self, email: &str, password: &str) -> Result<AuthLoginResult, AuthError> {
        let client = http_client(self.timeout_ms)?;
        let response = client
            .post(self.endpoint("auth/login"))
            .json(&serde_json::json!({ "email": email, "password": password }))
            .send()
            .await
            .map_err(|_| AuthError::Dependency("Failed to reach auth service.".to_string()))?;

        let payload = parse_json_response::<UpstreamLoginResponse>(response).await?;
        Ok(AuthLoginResult {
            requires_mfa: payload.requires_mfa,
            user: payload.user,
            access_token: payload.access_token,
        })
    }

    async fn validate(&self, access_token: &str) -> Result<AuthValidation, AuthError> {
        let client = http_client(self.timeout_ms)?;
        let response = client
            .post(self.endpoint("auth/validate"))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| AuthError::Dependency("Failed to reach auth service.".to_string()))?;

        let payload = parse_json_response::<UpstreamValidateResponse>(response).await?;

        Ok(AuthValidation {
            user_id: payload.user_id,
            name: payload.name,
            email: payload.email,
            email_verified: payload.email_verified,
            mfa_satisfied: payload.mfa_satisfied,
            session_expiry: payload.session_expiry,
            roles: payload.roles,
            permissions: payload.permissions,
        })
    }

    async fn logout(&self, access_token: &str) -> Result<(), AuthError> {
        let client = http_client(self.timeout_ms)?;
        let response = client
            .post(self.endpoint("auth/logout"))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| AuthError::Dependency("Failed to reach auth service.".to_string()))?;

        if response.status().is_success() {
            return Ok(());
        }

        let status =
            StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
        let message = extract_error_message(response)
            .await
            .unwrap_or_else(|| "Auth service rejected logout.".to_string());

        Err(AuthError::UpstreamClient { status, message })
    }
}

fn http_client(timeout_ms: u64) -> Result<Client, AuthError> {
    Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|_| AuthError::Dependency("Failed to initialize HTTP client.".to_string()))
}

async fn parse_json_response<T: for<'de> Deserialize<'de>>(
    response: reqwest::Response,
) -> Result<T, AuthError> {
    if response.status().is_success() {
        return response
            .json::<T>()
            .await
            .map_err(|_| AuthError::Dependency("Invalid response from auth service.".to_string()));
    }

    let status =
        StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let message = extract_error_message(response)
        .await
        .unwrap_or_else(|| "Auth service request failed.".to_string());
    Err(AuthError::UpstreamClient { status, message })
}

async fn extract_error_message(response: reqwest::Response) -> Option<String> {
    response
        .json::<ErrorEnvelope>()
        .await
        .ok()
        .and_then(|body| body.message)
}
