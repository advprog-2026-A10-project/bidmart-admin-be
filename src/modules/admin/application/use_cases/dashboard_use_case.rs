use std::sync::Arc;

use crate::modules::admin::application::dto::AdminDashboardSummaryDto;
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::AuthAdminRepository;

pub struct GetDashboardSummaryUseCase {
    auth_repo: Arc<dyn AuthAdminRepository>,
}

impl GetDashboardSummaryUseCase {
    pub fn new(auth_repo: Arc<dyn AuthAdminRepository>) -> Self {
        Self { auth_repo }
    }

    pub async fn execute(&self) -> Result<AdminDashboardSummaryDto, AdminError> {
        let summary = self.auth_repo.get_dashboard_summary().await?;
        Ok(AdminDashboardSummaryDto::from(summary))
    }
}
