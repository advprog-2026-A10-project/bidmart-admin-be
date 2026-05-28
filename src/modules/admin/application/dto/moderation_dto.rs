use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::modules::admin::domain::entities::ModerationListing;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModerationListingDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_path: String,
    pub seller_name: String,
    pub seller_id: Uuid,
    pub starting_price: i64,
    pub reserve_price: Option<i64>,
    pub current_bid: Option<i64>,
    pub bid_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
    pub thumbnail_url: String,
}

impl From<ModerationListing> for ModerationListingDto {
    fn from(value: ModerationListing) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            category_path: value.category_name,
            seller_name: value.seller_name,
            seller_id: value.seller_id,
            starting_price: value.start_price,
            reserve_price: value.reserve_price,
            current_bid: value.current_bid,
            bid_count: value.bid_count,
            status: normalize_listing_status(&value.listing_status),
            created_at: value.created_at,
            end_at: value.end_at,
            thumbnail_url: value.thumbnail_url,
        }
    }
}

fn normalize_listing_status(value: &str) -> String {
    match value.to_ascii_uppercase().as_str() {
        "SOLD" | "CANCELLED" | "EXPIRED" => "CLOSED".to_string(),
        other => other.to_string(),
    }
}
