use std::sync::Arc;

use uuid::Uuid;

use crate::modules::admin::application::dto::{ManagedUserDto, ManagedUserSessionDto};
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::AuthAdminRepository;

pub struct UserManagementUseCase {
    auth_repo: Arc<dyn AuthAdminRepository>,
}

impl UserManagementUseCase {
    pub fn new(auth_repo: Arc<dyn AuthAdminRepository>) -> Self {
        Self { auth_repo }
    }

    pub async fn list_users(&self) -> Result<Vec<ManagedUserDto>, AdminError> {
        let users = self.auth_repo.list_managed_users().await?;
        Ok(users.into_iter().map(ManagedUserDto::from).collect())
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<ManagedUserDto, AdminError> {
        let user = self
            .auth_repo
            .get_managed_user(user_id)
            .await?
            .ok_or_else(|| AdminError::NotFound("User not found.".to_string()))?;

        Ok(ManagedUserDto::from(user))
    }

    pub async fn list_user_sessions(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ManagedUserSessionDto>, AdminError> {
        let user_exists = self.auth_repo.get_managed_user(user_id).await?.is_some();
        if !user_exists {
            return Err(AdminError::NotFound("User not found.".to_string()));
        }

        let sessions = self.auth_repo.list_user_sessions(user_id).await?;
        Ok(sessions
            .into_iter()
            .map(ManagedUserSessionDto::from)
            .collect())
    }
}
