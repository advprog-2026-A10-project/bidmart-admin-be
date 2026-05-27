use async_trait::async_trait;
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    AdminCategory, Dispute, DisputeResolutionOutcome, ModerationListing, SystemActivitySnapshot,
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
    async fn list_categories(&self) -> Result<Vec<AdminCategory>, AdminError>;
    async fn create_category(
        &self,
        name: &str,
        slug: &str,
        parent_id: Option<i32>,
        image_url: Option<String>,
    ) -> Result<AdminCategory, AdminError>;
    async fn update_category(
        &self,
        category_id: i32,
        name: &str,
        slug: &str,
        parent_id: Option<i32>,
        image_url: Option<String>,
    ) -> Result<AdminCategory, AdminError>;
    async fn delete_category(&self, category_id: i32) -> Result<(), AdminError>;
    async fn get_system_activity_snapshot(&self) -> Result<SystemActivitySnapshot, AdminError>;
}
