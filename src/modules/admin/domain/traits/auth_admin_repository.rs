use async_trait::async_trait;
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    AdminDashboardSummary, ManagedUser, ManagedUserSession,
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
}
