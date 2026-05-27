use std::sync::Arc;

use uuid::Uuid;

use crate::modules::admin::application::dto::ModerationListingDto;
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::CoreAdminRepository;

pub struct ModerationUseCase {
    core_repo: Arc<dyn CoreAdminRepository>,
}

impl ModerationUseCase {
    pub fn new(core_repo: Arc<dyn CoreAdminRepository>) -> Self {
        Self { core_repo }
    }

    pub async fn list_listings(&self) -> Result<Vec<ModerationListingDto>, AdminError> {
        let listings = self.core_repo.list_moderation_listings().await?;
        Ok(listings
            .into_iter()
            .map(ModerationListingDto::from)
            .collect())
    }

    pub async fn get_listing(&self, listing_id: Uuid) -> Result<ModerationListingDto, AdminError> {
        let listing = self
            .core_repo
            .get_moderation_listing(listing_id)
            .await?
            .ok_or_else(|| AdminError::NotFound("Listing not found.".to_string()))?;

        Ok(ModerationListingDto::from(listing))
    }
}
