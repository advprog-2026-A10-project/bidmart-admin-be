use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    AdminDashboardSummary, ManagedUser, ManagedUserSession,
};
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::AuthAdminRepository;

#[derive(Clone)]
pub struct SqlxAuthAdminRepository {
    pool: PgPool,
}

impl SqlxAuthAdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Clone, FromRow)]
struct AdminDashboardSummaryRow {
    total_users: i64,
    active_users: i64,
    disabled_users: i64,
    pending_users: i64,
    active_sessions: i64,
}

#[derive(Debug, Clone, FromRow)]
struct ManagedUserRow {
    id: Uuid,
    name: String,
    email: String,
    status: String,
    roles: Vec<String>,
    created_at: DateTime<Utc>,
    last_seen_at: Option<DateTime<Utc>>,
    active_sessions: i64,
    email_verified: bool,
    mfa_email_enabled: bool,
    mfa_totp_enabled: bool,
}

#[derive(Debug, Clone, FromRow)]
struct ManagedUserSessionRow {
    id: Uuid,
    device: String,
    browser: String,
    os: String,
    ip: String,
    location: String,
    mfa_satisfied: bool,
    created_at: DateTime<Utc>,
    last_active_at: DateTime<Utc>,
    expired_at: DateTime<Utc>,
}

impl From<AdminDashboardSummaryRow> for AdminDashboardSummary {
    fn from(value: AdminDashboardSummaryRow) -> Self {
        Self {
            total_users: value.total_users,
            active_users: value.active_users,
            disabled_users: value.disabled_users,
            pending_users: value.pending_users,
            active_sessions: value.active_sessions,
        }
    }
}

impl From<ManagedUserRow> for ManagedUser {
    fn from(value: ManagedUserRow) -> Self {
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

impl From<ManagedUserSessionRow> for ManagedUserSession {
    fn from(value: ManagedUserSessionRow) -> Self {
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
        }
    }
}

#[async_trait]
impl AuthAdminRepository for SqlxAuthAdminRepository {
    async fn get_dashboard_summary(&self) -> Result<AdminDashboardSummary, AdminError> {
        let row = sqlx::query_as::<_, AdminDashboardSummaryRow>(
            r#"
            SELECT
                COUNT(*)::bigint AS total_users,
                COUNT(*) FILTER (WHERE u.status = 'ACTIVE')::bigint AS active_users,
                COUNT(*) FILTER (WHERE u.status = 'DISABLED')::bigint AS disabled_users,
                COUNT(*) FILTER (WHERE u.status = 'PENDING_VERIFICATION')::bigint AS pending_users,
                (
                    SELECT COUNT(*)::bigint
                    FROM sessions s
                    WHERE s.expired_at > NOW()
                ) AS active_sessions
            FROM users u
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load dashboard summary.".to_string()))?;

        Ok(AdminDashboardSummary::from(row))
    }

    async fn list_managed_users(&self) -> Result<Vec<ManagedUser>, AdminError> {
        let rows = sqlx::query_as::<_, ManagedUserRow>(
            r#"
            SELECT
                u.id,
                COALESCE(
                    NULLIF(TRIM(CONCAT(COALESCE(p.first_name, ''), ' ', COALESCE(p.last_name, ''))), ''),
                    split_part(u.email, '@', 1)
                ) AS name,
                u.email,
                u.status::text AS status,
                COALESCE(
                    ARRAY_REMOVE(ARRAY_AGG(DISTINCT r.name), NULL),
                    ARRAY[]::text[]
                ) AS roles,
                u.created_at,
                MAX(s.last_active_at) AS last_seen_at,
                COUNT(*) FILTER (WHERE s.expired_at > NOW())::bigint AS active_sessions,
                (u.email_verified_at IS NOT NULL) AS email_verified,
                u.mfa_email_enabled,
                u.mfa_totp_enabled
            FROM users u
            LEFT JOIN user_profiles p ON p.user_id = u.id
            LEFT JOIN user_roles ur ON ur.user_id = u.id
            LEFT JOIN roles r ON r.id = ur.role_id
            LEFT JOIN sessions s ON s.user_id = u.id
            GROUP BY
                u.id, p.first_name, p.last_name, u.email, u.status, u.created_at,
                u.email_verified_at, u.mfa_email_enabled, u.mfa_totp_enabled
            ORDER BY u.created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load users.".to_string()))?;

        Ok(rows.into_iter().map(ManagedUser::from).collect())
    }

    async fn get_managed_user(&self, user_id: Uuid) -> Result<Option<ManagedUser>, AdminError> {
        let row = sqlx::query_as::<_, ManagedUserRow>(
            r#"
            SELECT
                u.id,
                COALESCE(
                    NULLIF(TRIM(CONCAT(COALESCE(p.first_name, ''), ' ', COALESCE(p.last_name, ''))), ''),
                    split_part(u.email, '@', 1)
                ) AS name,
                u.email,
                u.status::text AS status,
                COALESCE(
                    ARRAY_REMOVE(ARRAY_AGG(DISTINCT r.name), NULL),
                    ARRAY[]::text[]
                ) AS roles,
                u.created_at,
                MAX(s.last_active_at) AS last_seen_at,
                COUNT(*) FILTER (WHERE s.expired_at > NOW())::bigint AS active_sessions,
                (u.email_verified_at IS NOT NULL) AS email_verified,
                u.mfa_email_enabled,
                u.mfa_totp_enabled
            FROM users u
            LEFT JOIN user_profiles p ON p.user_id = u.id
            LEFT JOIN user_roles ur ON ur.user_id = u.id
            LEFT JOIN roles r ON r.id = ur.role_id
            LEFT JOIN sessions s ON s.user_id = u.id
            WHERE u.id = $1
            GROUP BY
                u.id, p.first_name, p.last_name, u.email, u.status, u.created_at,
                u.email_verified_at, u.mfa_email_enabled, u.mfa_totp_enabled
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load user detail.".to_string()))?;

        Ok(row.map(ManagedUser::from))
    }

    async fn list_user_sessions(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ManagedUserSession>, AdminError> {
        let rows = sqlx::query_as::<_, ManagedUserSessionRow>(
            r#"
            SELECT
                s.id,
                s.device,
                s.browser,
                s.os,
                s.ip,
                s.location,
                s.mfa_satisfied,
                s.created_at,
                s.last_active_at,
                s.expired_at
            FROM sessions s
            WHERE s.user_id = $1
            ORDER BY s.last_active_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load user sessions.".to_string()))?;

        Ok(rows.into_iter().map(ManagedUserSession::from).collect())
    }
}
