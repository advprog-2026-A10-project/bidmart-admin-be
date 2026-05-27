use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    SecurityLoginAuditEntry, SecurityOverview, SecurityPolicy, SystemSecuritySnapshot,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityPolicyDto {
    pub max_concurrent_sessions: i32,
    pub enforcement_mode: String,
    pub force_mfa_for_admin: bool,
    pub updated_at: DateTime<Utc>,
}

impl From<SecurityPolicy> for SecurityPolicyDto {
    fn from(value: SecurityPolicy) -> Self {
        Self {
            max_concurrent_sessions: value.max_concurrent_sessions,
            enforcement_mode: value.enforcement_mode,
            force_mfa_for_admin: value.force_mfa_for_admin,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityOverviewDto {
    pub active_sessions: i64,
    pub users_with_multiple_sessions: i64,
    pub mfa_satisfied_sessions: i64,
    pub mfa_unsatisfied_sessions: i64,
}

impl From<SecurityOverview> for SecurityOverviewDto {
    fn from(value: SecurityOverview) -> Self {
        Self {
            active_sessions: value.active_sessions,
            users_with_multiple_sessions: value.users_with_multiple_sessions,
            mfa_satisfied_sessions: value.mfa_satisfied_sessions,
            mfa_unsatisfied_sessions: value.mfa_unsatisfied_sessions,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityLoginAuditEntryDto {
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

impl From<SecurityLoginAuditEntry> for SecurityLoginAuditEntryDto {
    fn from(value: SecurityLoginAuditEntry) -> Self {
        Self {
            session_id: value.session_id,
            user_id: value.user_id,
            name: value.name,
            email: value.email,
            status: value.status,
            ip: value.ip,
            location: value.location,
            device: value.device,
            browser: value.browser,
            os: value.os,
            mfa_satisfied: value.mfa_satisfied,
            created_at: value.created_at,
            last_active_at: value.last_active_at,
            expired_at: value.expired_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRuntimeConfigDto {
    pub session_cookie_name: String,
    pub session_cookie_secure: bool,
    pub session_cookie_same_site: String,
    pub authz_cache_ttl_seconds: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemSecuritySnapshotDto {
    pub generated_at: DateTime<Utc>,
    pub policy: SecurityPolicyDto,
    pub overview: SecurityOverviewDto,
    pub runtime: SecurityRuntimeConfigDto,
    pub login_audit: Vec<SecurityLoginAuditEntryDto>,
}

impl SystemSecuritySnapshotDto {
    pub fn from_snapshot_and_runtime(
        snapshot: SystemSecuritySnapshot,
        runtime: SecurityRuntimeConfigDto,
    ) -> Self {
        Self {
            generated_at: Utc::now(),
            policy: SecurityPolicyDto::from(snapshot.policy),
            overview: SecurityOverviewDto::from(snapshot.overview),
            runtime,
            login_audit: snapshot
                .login_audit
                .into_iter()
                .map(SecurityLoginAuditEntryDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSecurityPolicyCommand {
    pub max_concurrent_sessions: i32,
    pub enforcement_mode: String,
    pub force_mfa_for_admin: bool,
}
