use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ManagedUser {
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

#[derive(Debug, Clone)]
pub struct ManagedUserSession {
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
}
