use std::sync::Arc;

use uuid::Uuid;

use crate::modules::admin::application::dto::{
    ManagedUserDto, ManagedUserSessionDto, SessionActionResultDto,
};
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

    pub async fn revoke_user_session(
        &self,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<SessionActionResultDto, AdminError> {
        let user_exists = self.auth_repo.get_managed_user(user_id).await?.is_some();
        if !user_exists {
            return Err(AdminError::NotFound("User not found.".to_string()));
        }

        self.auth_repo
            .revoke_user_session(user_id, session_id)
            .await?;

        Ok(SessionActionResultDto {
            message: "Session revoked.".to_string(),
            revoked_count: 1,
        })
    }

    pub async fn revoke_all_user_sessions(
        &self,
        user_id: Uuid,
    ) -> Result<SessionActionResultDto, AdminError> {
        let user_exists = self.auth_repo.get_managed_user(user_id).await?.is_some();
        if !user_exists {
            return Err(AdminError::NotFound("User not found.".to_string()));
        }

        let revoked_count = self.auth_repo.revoke_all_user_sessions(user_id).await?;
        Ok(SessionActionResultDto {
            message: if revoked_count > 0 {
                "Active sessions revoked.".to_string()
            } else {
                "No active sessions to revoke.".to_string()
            },
            revoked_count,
        })
    }

    pub async fn suspend_user(&self, user_id: Uuid) -> Result<SessionActionResultDto, AdminError> {
        let (changed, revoked_count) = self.auth_repo.suspend_user(user_id).await?;

        let message = if changed {
            if revoked_count > 0 {
                "User suspended and active sessions revoked.".to_string()
            } else {
                "User suspended. No active sessions found.".to_string()
            }
        } else if revoked_count > 0 {
            "User already suspended. Active sessions revoked.".to_string()
        } else {
            "User already suspended.".to_string()
        };

        Ok(SessionActionResultDto {
            message,
            revoked_count,
        })
    }

    pub async fn reactivate_user(
        &self,
        user_id: Uuid,
    ) -> Result<SessionActionResultDto, AdminError> {
        let changed = self.auth_repo.reactivate_user(user_id).await?;
        Ok(SessionActionResultDto {
            message: if changed {
                "User reactivated.".to_string()
            } else {
                "User is already active.".to_string()
            },
            revoked_count: 0,
        })
    }
}
