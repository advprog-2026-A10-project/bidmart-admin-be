use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Dispute {
    pub id: Uuid,
    pub order_id: Uuid,
    pub opened_by: Uuid,
    pub reason: String,
    pub description: String,
    pub status: String,
    pub resolution: Option<String>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub buyer_id: Uuid,
    pub buyer_name: String,
    pub seller_id: Uuid,
    pub seller_name: String,
    pub order_title: String,
    pub order_image_url: String,
    pub final_price: i64,
    pub order_status: String,
}

#[derive(Debug, Clone, Copy)]
pub enum DisputeResolutionOutcome {
    Buyer,
    Seller,
}

impl DisputeResolutionOutcome {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "BUYER" => Some(Self::Buyer),
            "SELLER" => Some(Self::Seller),
            _ => None,
        }
    }
}
