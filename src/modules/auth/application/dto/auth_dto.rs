use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::auth::domain::entities::AuthUser;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginCommand {
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDto {
    pub user: AuthUser,
    pub access_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateResponseDto {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub mfa_satisfied: bool,
    pub session_expiry: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminMeResponseDto {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub mfa_satisfied: bool,
    pub session_expiry: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponseDto {
    pub message: String,
}
