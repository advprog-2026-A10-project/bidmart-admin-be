use axum::http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ForwardError {
    Client(StatusCode, String),
    Dependency(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthServiceUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthServiceLoginResponse {
    pub requires_mfa: bool,
    #[serde(default)]
    pub user: Option<AuthServiceUser>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub ticket: Option<String>,
    #[serde(default)]
    pub mfa_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthServiceValidateResponse {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub mfa_satisfied: bool,
    pub session_expiry: String,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ErrorEnvelope {
    message: Option<String>,
}

pub async fn login_with_auth_service(
    auth_base_url: &str,
    timeout_ms: u64,
    email: &str,
    password: &str,
) -> Result<AuthServiceLoginResponse, ForwardError> {
    let client = http_client(timeout_ms)?;
    let endpoint = format!("{}/auth/login", auth_base_url.trim_end_matches('/'));
    let response = client
        .post(endpoint)
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .map_err(|_| ForwardError::Dependency("Failed to reach auth service.".to_string()))?;
    parse_json_response(response).await
}

pub async fn validate_with_token(
    auth_base_url: &str,
    timeout_ms: u64,
    access_token: &str,
) -> Result<AuthServiceValidateResponse, ForwardError> {
    let client = http_client(timeout_ms)?;
    let endpoint = format!("{}/auth/validate", auth_base_url.trim_end_matches('/'));
    let response = client
        .post(endpoint)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|_| ForwardError::Dependency("Failed to reach auth service.".to_string()))?;
    parse_json_response(response).await
}

pub async fn logout_with_token(
    auth_base_url: &str,
    timeout_ms: u64,
    access_token: &str,
) -> Result<(), ForwardError> {
    let client = http_client(timeout_ms)?;
    let endpoint = format!("{}/auth/logout", auth_base_url.trim_end_matches('/'));
    let response = client
        .post(endpoint)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|_| ForwardError::Dependency("Failed to reach auth service.".to_string()))?;

    if response.status().is_success() {
        return Ok(());
    }

    let status =
        StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let message = extract_error_message(response)
        .await
        .unwrap_or_else(|| "Auth service rejected logout.".to_string());
    Err(ForwardError::Client(status, message))
}

fn http_client(timeout_ms: u64) -> Result<Client, ForwardError> {
    Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|_| ForwardError::Dependency("Failed to initialize HTTP client.".to_string()))
}

async fn parse_json_response<T: for<'de> Deserialize<'de>>(
    response: reqwest::Response,
) -> Result<T, ForwardError> {
    if response.status().is_success() {
        return response.json::<T>().await.map_err(|_| {
            ForwardError::Dependency("Invalid response from auth service.".to_string())
        });
    }

    let status =
        StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let message = extract_error_message(response)
        .await
        .unwrap_or_else(|| "Auth service request failed.".to_string());
    Err(ForwardError::Client(status, message))
}

async fn extract_error_message(response: reqwest::Response) -> Option<String> {
    response
        .json::<ErrorEnvelope>()
        .await
        .ok()
        .and_then(|body| body.message)
}
