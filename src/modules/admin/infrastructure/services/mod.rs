use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::modules::admin::infrastructure::repositories;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminDashboardSummaryDto {
    pub total_users: i64,
    pub active_users: i64,
    pub disabled_users: i64,
    pub pending_users: i64,
    pub active_sessions: i64,
}

impl From<repositories::AdminDashboardSummaryRow> for AdminDashboardSummaryDto {
    fn from(row: repositories::AdminDashboardSummaryRow) -> Self {
        Self {
            total_users: row.total_users,
            active_users: row.active_users,
            disabled_users: row.disabled_users,
            pending_users: row.pending_users,
            active_sessions: row.active_sessions,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedUserDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub active_sessions: i64,
    pub email_verified: bool,
    pub mfa_email_enabled: bool,
    pub mfa_totp_enabled: bool,
}

impl From<repositories::ManagedUserRow> for ManagedUserDto {
    fn from(row: repositories::ManagedUserRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            email: row.email,
            status: row.status,
            roles: row.roles,
            created_at: row.created_at,
            last_seen_at: row.last_seen_at,
            active_sessions: row.active_sessions,
            email_verified: row.email_verified,
            mfa_email_enabled: row.mfa_email_enabled,
            mfa_totp_enabled: row.mfa_totp_enabled,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedUserSessionDto {
    pub id: Uuid,
    pub device: String,
    pub browser: String,
    pub os: String,
    pub ip: String,
    pub location: String,
    pub mfa_satisfied: bool,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
    pub status: String,
}

impl From<repositories::ManagedUserSessionRow> for ManagedUserSessionDto {
    fn from(row: repositories::ManagedUserSessionRow) -> Self {
        let status = if row.expired_at > Utc::now() {
            "ACTIVE".to_string()
        } else {
            "REVOKED".to_string()
        };

        Self {
            id: row.id,
            device: row.device,
            browser: row.browser,
            os: row.os,
            ip: row.ip,
            location: row.location,
            mfa_satisfied: row.mfa_satisfied,
            created_at: row.created_at,
            last_active_at: row.last_active_at,
            expired_at: row.expired_at,
            status,
        }
    }
}

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

impl From<repositories::ModerationListingRow> for ModerationListingDto {
    fn from(row: repositories::ModerationListingRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            description: row.description,
            category_path: row.category_name,
            seller_name: row.seller_name,
            seller_id: row.seller_id,
            starting_price: row.start_price,
            reserve_price: row.reserve_price,
            current_bid: row.current_bid,
            bid_count: row.bid_count,
            status: normalize_listing_status(&row.listing_status),
            created_at: row.created_at,
            end_at: row.end_at,
            thumbnail_url: row.thumbnail_url,
        }
    }
}

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

impl From<repositories::DisputeRow> for DisputeDto {
    fn from(row: repositories::DisputeRow) -> Self {
        let opened_by_party = if row.opened_by == row.buyer_id {
            "BUYER".to_string()
        } else if row.opened_by == row.seller_id {
            "SELLER".to_string()
        } else {
            "UNKNOWN".to_string()
        };

        Self {
            id: row.id,
            order_id: row.order_id,
            opened_by: row.opened_by,
            opened_by_party,
            reason: row.reason,
            description: row.description,
            status: row.status,
            resolution: row.resolution,
            created_at: row.created_at,
            resolved_at: row.resolved_at,
            buyer_id: row.buyer_id,
            buyer_name: row.buyer_name,
            seller_id: row.seller_id,
            seller_name: row.seller_name,
            order_title: row.order_title,
            order_image_url: row.order_image_url,
            final_price: row.final_price,
            order_status: row.order_status,
        }
    }
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

    fn to_repository(self) -> repositories::ResolveDisputeOutcome {
        match self {
            Self::Buyer => repositories::ResolveDisputeOutcome::Buyer,
            Self::Seller => repositories::ResolveDisputeOutcome::Seller,
        }
    }
}

#[derive(Debug)]
pub enum ResolveDisputeError {
    NotFound,
    AlreadyResolved,
    Database(sqlx::Error),
}

pub async fn get_dashboard_summary(pool: &PgPool) -> Result<AdminDashboardSummaryDto, sqlx::Error> {
    let row = repositories::get_dashboard_summary(pool).await?;
    Ok(AdminDashboardSummaryDto::from(row))
}

pub async fn list_managed_users(pool: &PgPool) -> Result<Vec<ManagedUserDto>, sqlx::Error> {
    let rows = repositories::list_managed_users(pool).await?;
    Ok(rows.into_iter().map(ManagedUserDto::from).collect())
}

pub async fn get_managed_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<ManagedUserDto>, sqlx::Error> {
    let row = repositories::find_managed_user_by_id(pool, user_id).await?;
    Ok(row.map(ManagedUserDto::from))
}

pub async fn list_user_sessions(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<ManagedUserSessionDto>, sqlx::Error> {
    let rows = repositories::list_user_sessions(pool, user_id).await?;
    Ok(rows.into_iter().map(ManagedUserSessionDto::from).collect())
}

pub async fn list_moderation_listings(
    pool: &PgPool,
) -> Result<Vec<ModerationListingDto>, sqlx::Error> {
    let rows = repositories::list_moderation_listings(pool).await?;
    Ok(rows.into_iter().map(ModerationListingDto::from).collect())
}

pub async fn get_moderation_listing(
    pool: &PgPool,
    listing_id: Uuid,
) -> Result<Option<ModerationListingDto>, sqlx::Error> {
    let row = repositories::find_moderation_listing_by_id(pool, listing_id).await?;
    Ok(row.map(ModerationListingDto::from))
}

pub async fn list_disputes(pool: &PgPool) -> Result<Vec<DisputeDto>, sqlx::Error> {
    let rows = repositories::list_disputes(pool).await?;
    Ok(rows.into_iter().map(DisputeDto::from).collect())
}

pub async fn get_dispute(
    pool: &PgPool,
    dispute_id: Uuid,
) -> Result<Option<DisputeDto>, sqlx::Error> {
    let row = repositories::find_dispute_by_id(pool, dispute_id).await?;
    Ok(row.map(DisputeDto::from))
}

pub async fn resolve_dispute(
    pool: &PgPool,
    dispute_id: Uuid,
    outcome: DisputeResolutionOutcome,
    resolution: &str,
) -> Result<DisputeDto, ResolveDisputeError> {
    let row = repositories::resolve_dispute(pool, dispute_id, outcome.to_repository(), resolution)
        .await
        .map_err(|error| match error {
            repositories::ResolveDisputeError::NotFound => ResolveDisputeError::NotFound,
            repositories::ResolveDisputeError::AlreadyResolved => {
                ResolveDisputeError::AlreadyResolved
            }
            repositories::ResolveDisputeError::Database(error) => {
                ResolveDisputeError::Database(error)
            }
        })?;

    Ok(DisputeDto::from(row))
}

fn normalize_listing_status(value: &str) -> String {
    match value.to_ascii_uppercase().as_str() {
        "SOLD" | "CANCELLED" | "EXPIRED" => "CLOSED".to_string(),
        other => other.to_string(),
    }
}
