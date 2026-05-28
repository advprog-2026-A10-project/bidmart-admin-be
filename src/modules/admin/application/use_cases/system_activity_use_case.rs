use std::sync::Arc;

use crate::modules::admin::application::dto::SystemActivitySnapshotDto;
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::CoreAdminRepository;

pub struct SystemActivityUseCase {
    core_repo: Arc<dyn CoreAdminRepository>,
}

impl SystemActivityUseCase {
    pub fn new(core_repo: Arc<dyn CoreAdminRepository>) -> Self {
        Self { core_repo }
    }

    pub async fn get_snapshot(&self) -> Result<SystemActivitySnapshotDto, AdminError> {
        let snapshot = self.core_repo.get_system_activity_snapshot().await?;
        Ok(SystemActivitySnapshotDto::from(snapshot))
    }
}
