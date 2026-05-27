use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    AdminDashboardSummary, ManagedUser, ManagedUserSession, RbacRole, RbacRoleDetail,
    RbacRoleMember, RbacUserAssignment, SecurityLoginAuditEntry, SecurityOverview, SecurityPolicy,
    SystemSecuritySnapshot,
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

#[derive(Debug, Clone, FromRow)]
struct RbacRoleRow {
    id: i32,
    name: String,
    permissions: Vec<String>,
    member_count: i64,
}

#[derive(Debug, Clone, FromRow)]
struct RbacRoleMemberRow {
    id: Uuid,
    name: String,
    email: String,
    status: String,
}

#[derive(Debug, Clone, FromRow)]
struct RbacUserAssignmentRow {
    id: Uuid,
    name: String,
    email: String,
    status: String,
    roles: Vec<String>,
}

#[derive(Debug, Clone, FromRow)]
struct SecurityPolicyRow {
    max_concurrent_sessions: i32,
    enforcement_mode: String,
    force_mfa_for_admin: bool,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
struct SecurityOverviewRow {
    active_sessions: i64,
    users_with_multiple_sessions: i64,
    mfa_satisfied_sessions: i64,
    mfa_unsatisfied_sessions: i64,
}

#[derive(Debug, Clone, FromRow)]
struct SecurityLoginAuditRow {
    session_id: Uuid,
    user_id: Uuid,
    name: String,
    email: String,
    status: String,
    ip: String,
    location: String,
    device: String,
    browser: String,
    os: String,
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

impl From<RbacRoleRow> for RbacRole {
    fn from(value: RbacRoleRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
            permissions: value.permissions,
            member_count: value.member_count,
        }
    }
}

impl From<RbacRoleMemberRow> for RbacRoleMember {
    fn from(value: RbacRoleMemberRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            status: value.status,
        }
    }
}

impl From<RbacUserAssignmentRow> for RbacUserAssignment {
    fn from(value: RbacUserAssignmentRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            status: value.status,
            roles: value.roles,
        }
    }
}

impl From<SecurityPolicyRow> for SecurityPolicy {
    fn from(value: SecurityPolicyRow) -> Self {
        Self {
            max_concurrent_sessions: value.max_concurrent_sessions,
            enforcement_mode: value.enforcement_mode,
            force_mfa_for_admin: value.force_mfa_for_admin,
            updated_at: value.updated_at,
        }
    }
}

impl From<SecurityOverviewRow> for SecurityOverview {
    fn from(value: SecurityOverviewRow) -> Self {
        Self {
            active_sessions: value.active_sessions,
            users_with_multiple_sessions: value.users_with_multiple_sessions,
            mfa_satisfied_sessions: value.mfa_satisfied_sessions,
            mfa_unsatisfied_sessions: value.mfa_unsatisfied_sessions,
        }
    }
}

impl From<SecurityLoginAuditRow> for SecurityLoginAuditEntry {
    fn from(value: SecurityLoginAuditRow) -> Self {
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

    async fn revoke_user_session(&self, user_id: Uuid, session_id: Uuid) -> Result<(), AdminError> {
        let now = Utc::now();

        let updated = sqlx::query(
            r#"
            UPDATE sessions
            SET expired_at = $3
            WHERE user_id = $1
              AND id = $2
              AND expired_at > $3
            "#,
        )
        .bind(user_id)
        .bind(session_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to revoke session.".to_string()))?;

        if updated.rows_affected() > 0 {
            return Ok(());
        }

        let existing = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM sessions
                WHERE user_id = $1
                  AND id = $2
            )
            "#,
        )
        .bind(user_id)
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to verify session.".to_string()))?;

        if !existing {
            return Err(AdminError::NotFound("Session not found.".to_string()));
        }

        Err(AdminError::Conflict("Session already revoked.".to_string()))
    }

    async fn revoke_all_user_sessions(&self, user_id: Uuid) -> Result<u64, AdminError> {
        let now = Utc::now();
        let updated = sqlx::query(
            r#"
            UPDATE sessions
            SET expired_at = $2
            WHERE user_id = $1
              AND expired_at > $2
            "#,
        )
        .bind(user_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to revoke sessions.".to_string()))?;

        Ok(updated.rows_affected())
    }

    async fn list_rbac_roles(&self) -> Result<Vec<RbacRole>, AdminError> {
        let rows = sqlx::query_as::<_, RbacRoleRow>(
            r#"
            SELECT
                r.id,
                r.name,
                COALESCE(
                    ARRAY_REMOVE(ARRAY_AGG(DISTINCT p.slug), NULL),
                    ARRAY[]::text[]
                ) AS permissions,
                COUNT(DISTINCT ur.user_id)::bigint AS member_count
            FROM roles r
            LEFT JOIN role_permissions rp ON rp.role_id = r.id
            LEFT JOIN permissions p ON p.id = rp.permission_id
            LEFT JOIN user_roles ur ON ur.role_id = r.id
            GROUP BY r.id, r.name
            ORDER BY r.name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load roles.".to_string()))?;

        Ok(rows.into_iter().map(RbacRole::from).collect())
    }

    async fn create_rbac_role(&self, role_name: &str) -> Result<RbacRole, AdminError> {
        let inserted = sqlx::query_scalar::<_, i32>(
            r#"
            INSERT INTO roles (name)
            VALUES ($1)
            ON CONFLICT (name) DO NOTHING
            RETURNING id
            "#,
        )
        .bind(role_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to create role.".to_string()))?;

        let Some(role_id) = inserted else {
            let duplicated = sqlx::query_scalar::<_, bool>(
                r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM roles
                    WHERE UPPER(name) = UPPER($1)
                )
                "#,
            )
            .bind(role_name)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AdminError::Repository("Failed to verify role.".to_string()))?;

            if duplicated {
                return Err(AdminError::Conflict("Role already exists.".to_string()));
            }

            return Err(AdminError::Repository("Failed to create role.".to_string()));
        };

        let role = sqlx::query_as::<_, RbacRoleRow>(
            r#"
            SELECT
                r.id,
                r.name,
                COALESCE(
                    ARRAY_REMOVE(ARRAY_AGG(DISTINCT p.slug), NULL),
                    ARRAY[]::text[]
                ) AS permissions,
                COUNT(DISTINCT ur.user_id)::bigint AS member_count
            FROM roles r
            LEFT JOIN role_permissions rp ON rp.role_id = r.id
            LEFT JOIN permissions p ON p.id = rp.permission_id
            LEFT JOIN user_roles ur ON ur.role_id = r.id
            WHERE r.id = $1
            GROUP BY r.id, r.name
            LIMIT 1
            "#,
        )
        .bind(role_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load created role.".to_string()))?;

        Ok(RbacRole::from(role))
    }

    async fn get_rbac_role_detail(
        &self,
        role_id: i32,
    ) -> Result<Option<RbacRoleDetail>, AdminError> {
        let role = sqlx::query_as::<_, RbacRoleRow>(
            r#"
            SELECT
                r.id,
                r.name,
                COALESCE(
                    ARRAY_REMOVE(ARRAY_AGG(DISTINCT p.slug), NULL),
                    ARRAY[]::text[]
                ) AS permissions,
                COUNT(DISTINCT ur.user_id)::bigint AS member_count
            FROM roles r
            LEFT JOIN role_permissions rp ON rp.role_id = r.id
            LEFT JOIN permissions p ON p.id = rp.permission_id
            LEFT JOIN user_roles ur ON ur.role_id = r.id
            WHERE r.id = $1
            GROUP BY r.id, r.name
            LIMIT 1
            "#,
        )
        .bind(role_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load role detail.".to_string()))?;

        let Some(role) = role else {
            return Ok(None);
        };

        let members = sqlx::query_as::<_, RbacRoleMemberRow>(
            r#"
            SELECT
                u.id,
                COALESCE(
                    NULLIF(TRIM(CONCAT(COALESCE(p.first_name, ''), ' ', COALESCE(p.last_name, ''))), ''),
                    split_part(u.email, '@', 1)
                ) AS name,
                u.email,
                u.status::text AS status
            FROM user_roles ur
            INNER JOIN users u ON u.id = ur.user_id
            LEFT JOIN user_profiles p ON p.user_id = u.id
            WHERE ur.role_id = $1
            ORDER BY u.created_at DESC
            "#,
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load role members.".to_string()))?;

        Ok(Some(RbacRoleDetail {
            id: role.id,
            name: role.name,
            permissions: role.permissions,
            members: members.into_iter().map(RbacRoleMember::from).collect(),
        }))
    }

    async fn list_rbac_users(&self) -> Result<Vec<RbacUserAssignment>, AdminError> {
        let rows = sqlx::query_as::<_, RbacUserAssignmentRow>(
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
                ) AS roles
            FROM users u
            LEFT JOIN user_profiles p ON p.user_id = u.id
            LEFT JOIN user_roles ur ON ur.user_id = u.id
            LEFT JOIN roles r ON r.id = ur.role_id
            GROUP BY u.id, p.first_name, p.last_name, u.email, u.status, u.created_at
            ORDER BY u.created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load RBAC users.".to_string()))?;

        Ok(rows.into_iter().map(RbacUserAssignment::from).collect())
    }

    async fn list_permissions(&self) -> Result<Vec<String>, AdminError> {
        let rows = sqlx::query_scalar::<_, String>(
            r#"
            SELECT slug
            FROM permissions
            ORDER BY slug ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load permissions.".to_string()))?;

        Ok(rows)
    }

    async fn assign_user_role(&self, user_id: Uuid, role_name: &str) -> Result<bool, AdminError> {
        let inserted = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO user_roles (user_id, role_id)
            SELECT u.id, r.id
            FROM users u
            JOIN roles r ON UPPER(r.name) = UPPER($2)
            WHERE u.id = $1
            ON CONFLICT (user_id, role_id) DO NOTHING
            RETURNING user_id
            "#,
        )
        .bind(user_id)
        .bind(role_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to assign user role.".to_string()))?;

        if inserted.is_some() {
            return Ok(true);
        }

        ensure_user_exists(&self.pool, user_id).await?;
        ensure_role_exists(&self.pool, role_name).await?;
        Ok(false)
    }

    async fn revoke_user_role(&self, user_id: Uuid, role_name: &str) -> Result<bool, AdminError> {
        let deleted_rows = sqlx::query(
            r#"
            DELETE FROM user_roles ur
            USING roles r
            WHERE ur.role_id = r.id
              AND ur.user_id = $1
              AND UPPER(r.name) = UPPER($2)
            "#,
        )
        .bind(user_id)
        .bind(role_name)
        .execute(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to revoke user role.".to_string()))?
        .rows_affected();

        if deleted_rows > 0 {
            return Ok(true);
        }

        ensure_user_exists(&self.pool, user_id).await?;
        ensure_role_exists(&self.pool, role_name).await?;
        Ok(false)
    }

    async fn assign_role_permission(
        &self,
        role_name: &str,
        permission: &str,
    ) -> Result<bool, AdminError> {
        let inserted = sqlx::query_scalar::<_, i32>(
            r#"
            INSERT INTO role_permissions (role_id, permission_id)
            SELECT r.id, p.id
            FROM roles r
            JOIN permissions p ON p.slug = $2
            WHERE UPPER(r.name) = UPPER($1)
            ON CONFLICT (role_id, permission_id) DO NOTHING
            RETURNING role_id
            "#,
        )
        .bind(role_name)
        .bind(permission)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to assign role permission.".to_string()))?;

        if inserted.is_some() {
            return Ok(true);
        }

        ensure_role_exists(&self.pool, role_name).await?;
        ensure_permission_exists(&self.pool, permission).await?;
        Ok(false)
    }

    async fn revoke_role_permission(
        &self,
        role_name: &str,
        permission: &str,
    ) -> Result<bool, AdminError> {
        let deleted_rows = sqlx::query(
            r#"
            DELETE FROM role_permissions rp
            USING roles r, permissions p
            WHERE rp.role_id = r.id
              AND rp.permission_id = p.id
              AND UPPER(r.name) = UPPER($1)
              AND p.slug = $2
            "#,
        )
        .bind(role_name)
        .bind(permission)
        .execute(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to revoke role permission.".to_string()))?
        .rows_affected();

        if deleted_rows > 0 {
            return Ok(true);
        }

        ensure_role_exists(&self.pool, role_name).await?;
        ensure_permission_exists(&self.pool, permission).await?;
        Ok(false)
    }

    async fn get_system_security_snapshot(&self) -> Result<SystemSecuritySnapshot, AdminError> {
        ensure_security_policy_table(&self.pool).await?;

        let policy_row = sqlx::query_as::<_, SecurityPolicyRow>(
            r#"
            SELECT
                max_concurrent_sessions,
                enforcement_mode,
                force_mfa_for_admin,
                updated_at
            FROM admin_security_policy
            WHERE id = 1
            LIMIT 1
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load security policy.".to_string()))?;

        let overview_row = sqlx::query_as::<_, SecurityOverviewRow>(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE s.expired_at > NOW())::bigint AS active_sessions,
                COUNT(*) FILTER (WHERE active_per_user.active_count > 1)::bigint AS users_with_multiple_sessions,
                COUNT(*) FILTER (WHERE s.expired_at > NOW() AND s.mfa_satisfied = TRUE)::bigint AS mfa_satisfied_sessions,
                COUNT(*) FILTER (WHERE s.expired_at > NOW() AND s.mfa_satisfied = FALSE)::bigint AS mfa_unsatisfied_sessions
            FROM sessions s
            LEFT JOIN (
                SELECT user_id, COUNT(*)::bigint AS active_count
                FROM sessions
                WHERE expired_at > NOW()
                GROUP BY user_id
            ) active_per_user ON active_per_user.user_id = s.user_id
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load security overview.".to_string()))?;

        let audit_rows = sqlx::query_as::<_, SecurityLoginAuditRow>(
            r#"
            SELECT
                s.id AS session_id,
                u.id AS user_id,
                COALESCE(
                    NULLIF(TRIM(CONCAT(COALESCE(p.first_name, ''), ' ', COALESCE(p.last_name, ''))), ''),
                    split_part(u.email, '@', 1)
                ) AS name,
                u.email,
                u.status::text AS status,
                s.ip,
                s.location,
                s.device,
                s.browser,
                s.os,
                s.mfa_satisfied,
                s.created_at,
                s.last_active_at,
                s.expired_at
            FROM sessions s
            INNER JOIN users u ON u.id = s.user_id
            LEFT JOIN user_profiles p ON p.user_id = u.id
            ORDER BY s.created_at DESC
            LIMIT 60
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load login audit.".to_string()))?;

        Ok(SystemSecuritySnapshot {
            policy: SecurityPolicy::from(policy_row),
            overview: SecurityOverview::from(overview_row),
            login_audit: audit_rows
                .into_iter()
                .map(SecurityLoginAuditEntry::from)
                .collect(),
        })
    }

    async fn update_security_policy(
        &self,
        max_concurrent_sessions: i32,
        enforcement_mode: &str,
        force_mfa_for_admin: bool,
    ) -> Result<SystemSecuritySnapshot, AdminError> {
        ensure_security_policy_table(&self.pool).await?;

        sqlx::query(
            r#"
            UPDATE admin_security_policy
            SET
                max_concurrent_sessions = $2,
                enforcement_mode = $3,
                force_mfa_for_admin = $4,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(1_i32)
        .bind(max_concurrent_sessions)
        .bind(enforcement_mode)
        .bind(force_mfa_for_admin)
        .execute(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to update security policy.".to_string()))?;

        self.get_system_security_snapshot().await
    }
}

async fn ensure_user_exists(pool: &PgPool, user_id: Uuid) -> Result<(), AdminError> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM users
            WHERE id = $1
        )
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|_| AdminError::Repository("Failed to verify user.".to_string()))?;

    if exists {
        Ok(())
    } else {
        Err(AdminError::NotFound("User not found.".to_string()))
    }
}

async fn ensure_role_exists(pool: &PgPool, role_name: &str) -> Result<(), AdminError> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM roles
            WHERE UPPER(name) = UPPER($1)
        )
        "#,
    )
    .bind(role_name)
    .fetch_one(pool)
    .await
    .map_err(|_| AdminError::Repository("Failed to verify role.".to_string()))?;

    if exists {
        Ok(())
    } else {
        Err(AdminError::NotFound("Role not found.".to_string()))
    }
}

async fn ensure_permission_exists(pool: &PgPool, permission: &str) -> Result<(), AdminError> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM permissions
            WHERE slug = $1
        )
        "#,
    )
    .bind(permission)
    .fetch_one(pool)
    .await
    .map_err(|_| AdminError::Repository("Failed to verify permission.".to_string()))?;

    if exists {
        Ok(())
    } else {
        Err(AdminError::NotFound("Permission not found.".to_string()))
    }
}

async fn ensure_security_policy_table(pool: &PgPool) -> Result<(), AdminError> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS admin_security_policy (
            id INTEGER PRIMARY KEY,
            max_concurrent_sessions INTEGER NOT NULL DEFAULT 3,
            enforcement_mode VARCHAR(20) NOT NULL DEFAULT 'REVOKE_OLDEST',
            force_mfa_for_admin BOOLEAN NOT NULL DEFAULT true,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            CONSTRAINT chk_admin_security_policy_mode CHECK (
                enforcement_mode IN ('REJECT_NEW', 'REVOKE_OLDEST')
            ),
            CONSTRAINT chk_admin_security_policy_max CHECK (
                max_concurrent_sessions >= 1
            )
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|_| AdminError::Repository("Failed to prepare security policy table.".to_string()))?;

    sqlx::query(
        r#"
        INSERT INTO admin_security_policy (
            id,
            max_concurrent_sessions,
            enforcement_mode,
            force_mfa_for_admin
        )
        VALUES (1, 3, 'REVOKE_OLDEST', TRUE)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .execute(pool)
    .await
    .map_err(|_| AdminError::Repository("Failed to seed security policy.".to_string()))?;

    Ok(())
}
