use async_trait::async_trait;
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    Dispute, DisputeResolutionOutcome, ModerationListing,
};
use crate::modules::admin::domain::errors::AdminError;

#[async_trait]
pub trait CoreAdminRepository: Send + Sync {
    async fn list_moderation_listings(&self) -> Result<Vec<ModerationListing>, AdminError>;
    async fn get_moderation_listing(
        &self,
        listing_id: Uuid,
    ) -> Result<Option<ModerationListing>, AdminError>;
    async fn list_disputes(&self) -> Result<Vec<Dispute>, AdminError>;
    async fn get_dispute(&self, dispute_id: Uuid) -> Result<Option<Dispute>, AdminError>;
    async fn resolve_dispute(
        &self,
        dispute_id: Uuid,
        outcome: DisputeResolutionOutcome,
        resolution: &str,
    ) -> Result<Dispute, AdminError>;
}
