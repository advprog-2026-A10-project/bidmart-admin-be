use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ModerationListing {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category_name: String,
    pub seller_name: String,
    pub seller_id: Uuid,
    pub start_price: i64,
    pub reserve_price: Option<i64>,
    pub current_bid: Option<i64>,
    pub bid_count: i32,
    pub listing_status: String,
    pub created_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
    pub thumbnail_url: String,
}
