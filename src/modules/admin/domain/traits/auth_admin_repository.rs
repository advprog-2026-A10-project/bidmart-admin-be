use async_trait::async_trait;
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    AdminDashboardSummary, ManagedUser, ManagedUserSession, RbacRole, RbacRoleDetail,
    RbacUserAssignment, SystemSecuritySnapshot,
};
use crate::modules::admin::domain::errors::AdminError;

#[async_trait]
pub trait AuthAdminRepository: Send + Sync {
    async fn get_dashboard_summary(&self) -> Result<AdminDashboardSummary, AdminError>;
    async fn list_managed_users(&self) -> Result<Vec<ManagedUser>, AdminError>;
    async fn get_managed_user(&self, user_id: Uuid) -> Result<Option<ManagedUser>, AdminError>;
    async fn list_user_sessions(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ManagedUserSession>, AdminError>;
    async fn revoke_user_session(&self, user_id: Uuid, session_id: Uuid) -> Result<(), AdminError>;
    async fn revoke_all_user_sessions(&self, user_id: Uuid) -> Result<u64, AdminError>;
    async fn list_rbac_roles(&self) -> Result<Vec<RbacRole>, AdminError>;
    async fn create_rbac_role(&self, role_name: &str) -> Result<RbacRole, AdminError>;
    async fn get_rbac_role_detail(
        &self,
        role_id: i32,
    ) -> Result<Option<RbacRoleDetail>, AdminError>;
    async fn list_rbac_users(&self) -> Result<Vec<RbacUserAssignment>, AdminError>;
    async fn list_permissions(&self) -> Result<Vec<String>, AdminError>;
    async fn assign_user_role(&self, user_id: Uuid, role_name: &str) -> Result<bool, AdminError>;
    async fn revoke_user_role(&self, user_id: Uuid, role_name: &str) -> Result<bool, AdminError>;
    async fn assign_role_permission(
        &self,
        role_name: &str,
        permission: &str,
    ) -> Result<bool, AdminError>;
    async fn revoke_role_permission(
        &self,
        role_name: &str,
        permission: &str,
    ) -> Result<bool, AdminError>;
    async fn get_system_security_snapshot(&self) -> Result<SystemSecuritySnapshot, AdminError>;
    async fn update_security_policy(
        &self,
        max_concurrent_sessions: i32,
        enforcement_mode: &str,
        force_mfa_for_admin: bool,
    ) -> Result<SystemSecuritySnapshot, AdminError>;
}
