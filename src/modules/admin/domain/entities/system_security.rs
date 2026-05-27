use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub max_concurrent_sessions: i32,
    pub enforcement_mode: String,
    pub force_mfa_for_admin: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SecurityOverview {
    pub active_sessions: i64,
    pub users_with_multiple_sessions: i64,
    pub mfa_satisfied_sessions: i64,
    pub mfa_unsatisfied_sessions: i64,
}

#[derive(Debug, Clone)]
pub struct SecurityLoginAuditEntry {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub ip: String,
    pub location: String,
    pub device: String,
    pub browser: String,
    pub os: String,
    pub mfa_satisfied: bool,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SystemSecuritySnapshot {
    pub policy: SecurityPolicy,
    pub overview: SecurityOverview,
    pub login_audit: Vec<SecurityLoginAuditEntry>,
}
