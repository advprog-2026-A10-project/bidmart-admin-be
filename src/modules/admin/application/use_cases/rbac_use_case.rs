use std::sync::Arc;

use uuid::Uuid;

use crate::modules::admin::application::dto::{
    RbacMutationResultDto, RbacPermissionsPanelDto, RbacRoleDetailDto, RbacRoleDto,
    RbacUserAssignmentDto,
};
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::AuthAdminRepository;

pub struct RbacUseCase {
    auth_repo: Arc<dyn AuthAdminRepository>,
}

impl RbacUseCase {
    pub fn new(auth_repo: Arc<dyn AuthAdminRepository>) -> Self {
        Self { auth_repo }
    }

    pub async fn list_roles(&self) -> Result<Vec<RbacRoleDto>, AdminError> {
        let roles = self.auth_repo.list_rbac_roles().await?;
        Ok(roles.into_iter().map(RbacRoleDto::from).collect())
    }

    pub async fn create_role(&self, name: &str) -> Result<RbacRoleDto, AdminError> {
        let normalized = normalize_role_name(name)?;
        let role = self.auth_repo.create_rbac_role(&normalized).await?;
        Ok(RbacRoleDto::from(role))
    }

    pub async fn get_role_detail(&self, role_id: i32) -> Result<RbacRoleDetailDto, AdminError> {
        if role_id <= 0 {
            return Err(AdminError::InvalidInput("Invalid role id.".to_string()));
        }

        let role = self
            .auth_repo
            .get_rbac_role_detail(role_id)
            .await?
            .ok_or_else(|| AdminError::NotFound("Role not found.".to_string()))?;
        Ok(RbacRoleDetailDto::from(role))
    }

    pub async fn get_permissions_panel(&self) -> Result<RbacPermissionsPanelDto, AdminError> {
        let roles = self.auth_repo.list_rbac_roles().await?;
        let users = self.auth_repo.list_rbac_users().await?;
        let permissions = self.auth_repo.list_permissions().await?;

        Ok(RbacPermissionsPanelDto {
            roles: roles.into_iter().map(RbacRoleDto::from).collect(),
            users: users.into_iter().map(RbacUserAssignmentDto::from).collect(),
            permissions,
        })
    }

    pub async fn assign_user_role(
        &self,
        user_id: Uuid,
        role_name: &str,
    ) -> Result<RbacMutationResultDto, AdminError> {
        let normalized = normalize_role_name(role_name)?;
        let changed = self
            .auth_repo
            .assign_user_role(user_id, &normalized)
            .await?;
        Ok(RbacMutationResultDto {
            message: if changed {
                "Role assigned to user.".to_string()
            } else {
                "Role assignment unchanged.".to_string()
            },
            changed,
        })
    }

    pub async fn revoke_user_role(
        &self,
        user_id: Uuid,
        role_name: &str,
    ) -> Result<RbacMutationResultDto, AdminError> {
        let normalized = normalize_role_name(role_name)?;
        let changed = self
            .auth_repo
            .revoke_user_role(user_id, &normalized)
            .await?;
        Ok(RbacMutationResultDto {
            message: if changed {
                "Role revoked from user.".to_string()
            } else {
                "Role revoke unchanged.".to_string()
            },
            changed,
        })
    }

    pub async fn assign_role_permission(
        &self,
        role_name: &str,
        permission: &str,
    ) -> Result<RbacMutationResultDto, AdminError> {
        let normalized_role = normalize_role_name(role_name)?;
        let normalized_permission = normalize_permission(permission)?;
        let changed = self
            .auth_repo
            .assign_role_permission(&normalized_role, &normalized_permission)
            .await?;

        Ok(RbacMutationResultDto {
            message: if changed {
                "Permission assigned to role.".to_string()
            } else {
                "Role permission assignment unchanged.".to_string()
            },
            changed,
        })
    }

    pub async fn revoke_role_permission(
        &self,
        role_name: &str,
        permission: &str,
    ) -> Result<RbacMutationResultDto, AdminError> {
        let normalized_role = normalize_role_name(role_name)?;
        let normalized_permission = normalize_permission(permission)?;
        let changed = self
            .auth_repo
            .revoke_role_permission(&normalized_role, &normalized_permission)
            .await?;

        Ok(RbacMutationResultDto {
            message: if changed {
                "Permission revoked from role.".to_string()
            } else {
                "Role permission revoke unchanged.".to_string()
            },
            changed,
        })
    }
}

fn normalize_role_name(value: &str) -> Result<String, AdminError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AdminError::InvalidInput("Role is required.".to_string()));
    }
    Ok(trimmed.to_ascii_uppercase())
}

fn normalize_permission(value: &str) -> Result<String, AdminError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AdminError::InvalidInput(
            "Permission is required.".to_string(),
        ));
    }
    Ok(trimmed.to_ascii_lowercase())
}
