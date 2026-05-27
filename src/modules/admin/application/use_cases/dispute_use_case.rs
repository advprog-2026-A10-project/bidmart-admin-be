use std::sync::Arc;

use uuid::Uuid;

use crate::modules::admin::application::dto::DisputeDto;
use crate::modules::admin::domain::entities::DisputeResolutionOutcome;
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::CoreAdminRepository;

pub struct DisputeUseCase {
    core_repo: Arc<dyn CoreAdminRepository>,
}

impl DisputeUseCase {
    pub fn new(core_repo: Arc<dyn CoreAdminRepository>) -> Self {
        Self { core_repo }
    }

    pub async fn list_disputes(&self) -> Result<Vec<DisputeDto>, AdminError> {
        let disputes = self.core_repo.list_disputes().await?;
        Ok(disputes.into_iter().map(DisputeDto::from).collect())
    }

    pub async fn get_dispute(&self, dispute_id: Uuid) -> Result<DisputeDto, AdminError> {
        let dispute = self
            .core_repo
            .get_dispute(dispute_id)
            .await?
            .ok_or_else(|| AdminError::NotFound("Dispute not found.".to_string()))?;

        Ok(DisputeDto::from(dispute))
    }

    pub async fn resolve_dispute(
        &self,
        dispute_id: Uuid,
        outcome: DisputeResolutionOutcome,
        resolution: &str,
    ) -> Result<DisputeDto, AdminError> {
        let resolution = resolution.trim();
        if resolution.is_empty() {
            return Err(AdminError::InvalidInput(
                "Resolution is required.".to_string(),
            ));
        }

        let dispute = self
            .core_repo
            .resolve_dispute(dispute_id, outcome, resolution)
            .await?;

        Ok(DisputeDto::from(dispute))
    }
}
