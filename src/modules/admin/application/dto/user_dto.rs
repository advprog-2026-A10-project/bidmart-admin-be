use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::modules::admin::domain::entities::{ManagedUser, ManagedUserSession};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedUserDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub active_sessions: i64,
    pub email_verified: bool,
    pub mfa_email_enabled: bool,
    pub mfa_totp_enabled: bool,
}

impl From<ManagedUser> for ManagedUserDto {
    fn from(value: ManagedUser) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            status: value.status,
            roles: value.roles,
            created_at: value.created_at,
            last_seen_at: value.last_seen_at,
            active_sessions: value.active_sessions,
            email_verified: value.email_verified,
            mfa_email_enabled: value.mfa_email_enabled,
            mfa_totp_enabled: value.mfa_totp_enabled,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedUserSessionDto {
    pub id: Uuid,
    pub device: String,
    pub browser: String,
    pub os: String,
    pub ip: String,
    pub location: String,
    pub mfa_satisfied: bool,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
    pub status: String,
}

impl From<ManagedUserSession> for ManagedUserSessionDto {
    fn from(value: ManagedUserSession) -> Self {
        let status = if value.expired_at > Utc::now() {
            "ACTIVE".to_string()
        } else {
            "REVOKED".to_string()
        };

        Self {
            id: value.id,
            device: value.device,
            browser: value.browser,
            os: value.os,
            ip: value.ip,
            location: value.location,
            mfa_satisfied: value.mfa_satisfied,
            created_at: value.created_at,
            last_active_at: value.last_active_at,
            expired_at: value.expired_at,
            status,
        }
    }
}
