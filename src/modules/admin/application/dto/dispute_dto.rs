use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::admin::domain::entities::Dispute;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisputeDto {
    pub id: Uuid,
    pub order_id: Uuid,
    pub opened_by: Uuid,
    pub opened_by_party: String,
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

impl From<Dispute> for DisputeDto {
    fn from(value: Dispute) -> Self {
        let opened_by_party = if value.opened_by == value.buyer_id {
            "BUYER".to_string()
        } else if value.opened_by == value.seller_id {
            "SELLER".to_string()
        } else {
            "UNKNOWN".to_string()
        };

        Self {
            id: value.id,
            order_id: value.order_id,
            opened_by: value.opened_by,
            opened_by_party,
            reason: value.reason,
            description: value.description,
            status: value.status,
            resolution: value.resolution,
            created_at: value.created_at,
            resolved_at: value.resolved_at,
            buyer_id: value.buyer_id,
            buyer_name: value.buyer_name,
            seller_id: value.seller_id,
            seller_name: value.seller_name,
            order_title: value.order_title,
            order_image_url: value.order_image_url,
            final_price: value.final_price,
            order_status: value.order_status,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveDisputeCommand {
    pub outcome: String,
    pub resolution: String,
}
