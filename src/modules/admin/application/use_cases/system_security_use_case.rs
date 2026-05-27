use std::sync::Arc;

use crate::modules::admin::application::dto::SystemSecuritySnapshotDto;
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::AuthAdminRepository;

pub struct SystemSecurityUseCase {
    auth_repo: Arc<dyn AuthAdminRepository>,
}

impl SystemSecurityUseCase {
    pub fn new(auth_repo: Arc<dyn AuthAdminRepository>) -> Self {
        Self { auth_repo }
    }

    pub async fn get_snapshot(
        &self,
    ) -> Result<crate::modules::admin::domain::entities::SystemSecuritySnapshot, AdminError> {
        self.auth_repo.get_system_security_snapshot().await
    }

    pub async fn update_policy(
        &self,
        max_concurrent_sessions: i32,
        enforcement_mode: &str,
        force_mfa_for_admin: bool,
    ) -> Result<crate::modules::admin::domain::entities::SystemSecuritySnapshot, AdminError> {
        if max_concurrent_sessions < 1 {
            return Err(AdminError::InvalidInput(
                "Max concurrent sessions must be at least 1.".to_string(),
            ));
        }

        let normalized_mode = enforcement_mode.trim().to_ascii_uppercase();
        if normalized_mode != "REJECT_NEW" && normalized_mode != "REVOKE_OLDEST" {
            return Err(AdminError::InvalidInput(
                "Invalid enforcement mode. Use REJECT_NEW or REVOKE_OLDEST.".to_string(),
            ));
        }

        self.auth_repo
            .update_security_policy(
                max_concurrent_sessions,
                &normalized_mode,
                force_mfa_for_admin,
            )
            .await
    }

    pub fn compose_snapshot_dto(
        &self,
        snapshot: crate::modules::admin::domain::entities::SystemSecuritySnapshot,
        runtime: crate::modules::admin::application::dto::SecurityRuntimeConfigDto,
    ) -> SystemSecuritySnapshotDto {
        SystemSecuritySnapshotDto::from_snapshot_and_runtime(snapshot, runtime)
    }
}
