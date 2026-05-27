use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    Dispute, DisputeResolutionOutcome, ModerationListing, SystemActivityEvent, SystemActivityKpi,
    SystemActivitySnapshot,
};
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::CoreAdminRepository;

#[derive(Clone)]
pub struct SqlxCoreAdminRepository {
    pool: PgPool,
}

impl SqlxCoreAdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Clone, FromRow)]
struct ModerationListingRow {
    id: Uuid,
    title: String,
    description: String,
    category_name: String,
    seller_name: String,
    seller_id: Uuid,
    start_price: i64,
    reserve_price: Option<i64>,
    current_bid: Option<i64>,
    bid_count: i32,
    listing_status: String,
    created_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
    thumbnail_url: String,
}

#[derive(Debug, Clone, FromRow)]
struct DisputeRow {
    id: Uuid,
    order_id: Uuid,
    opened_by: Uuid,
    reason: String,
    description: String,
    status: String,
    resolution: Option<String>,
    created_at: DateTime<Utc>,
    resolved_at: Option<DateTime<Utc>>,
    buyer_id: Uuid,
    buyer_name: String,
    seller_id: Uuid,
    seller_name: String,
    order_title: String,
    order_image_url: String,
    final_price: i64,
    order_status: String,
}

#[derive(Debug, Clone, FromRow)]
struct SystemActivityKpiRow {
    active_auctions: i64,
    bids_last_24h: i64,
    open_disputes: i64,
    published_events_last_24h: i64,
}

#[derive(Debug, Clone, FromRow)]
struct SystemActivityEventRow {
    kind: String,
    title: String,
    detail: String,
    occurred_at: DateTime<Utc>,
}

impl From<ModerationListingRow> for ModerationListing {
    fn from(value: ModerationListingRow) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            category_name: value.category_name,
            seller_name: value.seller_name,
            seller_id: value.seller_id,
            start_price: value.start_price,
            reserve_price: value.reserve_price,
            current_bid: value.current_bid,
            bid_count: value.bid_count,
            listing_status: value.listing_status,
            created_at: value.created_at,
            end_at: value.end_at,
            thumbnail_url: value.thumbnail_url,
        }
    }
}

impl From<DisputeRow> for Dispute {
    fn from(value: DisputeRow) -> Self {
        Self {
            id: value.id,
            order_id: value.order_id,
            opened_by: value.opened_by,
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

impl From<SystemActivityKpiRow> for SystemActivityKpi {
    fn from(value: SystemActivityKpiRow) -> Self {
        Self {
            active_auctions: value.active_auctions,
            bids_last_24h: value.bids_last_24h,
            open_disputes: value.open_disputes,
            published_events_last_24h: value.published_events_last_24h,
        }
    }
}

impl From<SystemActivityEventRow> for SystemActivityEvent {
    fn from(value: SystemActivityEventRow) -> Self {
        Self {
            kind: value.kind,
            title: value.title,
            detail: value.detail,
            occurred_at: value.occurred_at,
        }
    }
}

#[async_trait]
impl CoreAdminRepository for SqlxCoreAdminRepository {
    async fn list_moderation_listings(&self) -> Result<Vec<ModerationListing>, AdminError> {
        let rows = sqlx::query_as::<_, ModerationListingRow>(
            r#"
            SELECT
                l.id,
                l.title,
                l.description,
                l.category_name,
                l.seller_name,
                l.seller_id,
                l.start_price,
                l.reserve_price,
                CASE WHEN l.bid_count > 0 THEN l.current_price ELSE NULL END AS current_bid,
                l.bid_count,
                l.status::text AS listing_status,
                l.created_at,
                l.ends_at AS end_at,
                COALESCE(
                    (
                        SELECT li.url
                        FROM listing_images li
                        WHERE li.listing_id = l.id
                        ORDER BY li."order" ASC, li.created_at ASC
                        LIMIT 1
                    ),
                    ''
                ) AS thumbnail_url
            FROM listings l
            ORDER BY l.created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load listings.".to_string()))?;

        Ok(rows.into_iter().map(ModerationListing::from).collect())
    }

    async fn get_moderation_listing(
        &self,
        listing_id: Uuid,
    ) -> Result<Option<ModerationListing>, AdminError> {
        let row = sqlx::query_as::<_, ModerationListingRow>(
            r#"
            SELECT
                l.id,
                l.title,
                l.description,
                l.category_name,
                l.seller_name,
                l.seller_id,
                l.start_price,
                l.reserve_price,
                CASE WHEN l.bid_count > 0 THEN l.current_price ELSE NULL END AS current_bid,
                l.bid_count,
                l.status::text AS listing_status,
                l.created_at,
                l.ends_at AS end_at,
                COALESCE(
                    (
                        SELECT li.url
                        FROM listing_images li
                        WHERE li.listing_id = l.id
                        ORDER BY li."order" ASC, li.created_at ASC
                        LIMIT 1
                    ),
                    ''
                ) AS thumbnail_url
            FROM listings l
            WHERE l.id = $1
            LIMIT 1
            "#,
        )
        .bind(listing_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load listing detail.".to_string()))?;

        Ok(row.map(ModerationListing::from))
    }

    async fn list_disputes(&self) -> Result<Vec<Dispute>, AdminError> {
        let rows = sqlx::query_as::<_, DisputeRow>(
            r#"
            SELECT
                d.id,
                d.order_id,
                d.opened_by,
                d.reason::text AS reason,
                d.description,
                d.status::text AS status,
                d.resolution,
                d.created_at,
                d.resolved_at,
                o.buyer_id,
                o.buyer_name,
                o.seller_id,
                o.seller_name,
                o.title AS order_title,
                o.image_url AS order_image_url,
                o.final_price,
                o.status::text AS order_status
            FROM disputes d
            INNER JOIN orders o ON o.id = d.order_id
            ORDER BY d.created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load disputes.".to_string()))?;

        Ok(rows.into_iter().map(Dispute::from).collect())
    }

    async fn get_dispute(&self, dispute_id: Uuid) -> Result<Option<Dispute>, AdminError> {
        let row = sqlx::query_as::<_, DisputeRow>(
            r#"
            SELECT
                d.id,
                d.order_id,
                d.opened_by,
                d.reason::text AS reason,
                d.description,
                d.status::text AS status,
                d.resolution,
                d.created_at,
                d.resolved_at,
                o.buyer_id,
                o.buyer_name,
                o.seller_id,
                o.seller_name,
                o.title AS order_title,
                o.image_url AS order_image_url,
                o.final_price,
                o.status::text AS order_status
            FROM disputes d
            INNER JOIN orders o ON o.id = d.order_id
            WHERE d.id = $1
            LIMIT 1
            "#,
        )
        .bind(dispute_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load dispute detail.".to_string()))?;

        Ok(row.map(Dispute::from))
    }

    async fn get_system_activity_snapshot(&self) -> Result<SystemActivitySnapshot, AdminError> {
        let kpi_row = sqlx::query_as::<_, SystemActivityKpiRow>(
            r#"
            SELECT
                (
                    SELECT COUNT(*)::bigint
                    FROM auctions a
                    WHERE a.status IN ('ACTIVE'::auction_status, 'EXTENDED'::auction_status)
                ) AS active_auctions,
                (
                    SELECT COUNT(*)::bigint
                    FROM bids b
                    WHERE b.created_at >= NOW() - INTERVAL '24 hours'
                ) AS bids_last_24h,
                (
                    SELECT COUNT(*)::bigint
                    FROM disputes d
                    WHERE d.status IN ('OPEN'::dispute_status, 'UNDER_REVIEW'::dispute_status)
                ) AS open_disputes,
                (
                    SELECT COUNT(*)::bigint
                    FROM notifications n
                    WHERE n.created_at >= NOW() - INTERVAL '24 hours'
                      AND n.type::text IN ('BID_PLACED', 'WINNER_DETERMINED', 'ORDER_UPDATE', 'SYSTEM')
                ) AS published_events_last_24h
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load activity KPI.".to_string()))?;

        let event_rows = sqlx::query_as::<_, SystemActivityEventRow>(
            r#"
            SELECT
                events.kind,
                events.title,
                events.detail,
                events.occurred_at
            FROM (
                SELECT
                    'BID'::text AS kind,
                    COALESCE(a.title, 'Auction Bid') AS title,
                    CONCAT('Bid ', b.amount::text, ' by ', b.bidder_name) AS detail,
                    b.created_at AS occurred_at
                FROM bids b
                INNER JOIN auctions a ON a.id = b.auction_id

                UNION ALL

                SELECT
                    'DISPUTE'::text AS kind,
                    o.title AS title,
                    CONCAT('Dispute status: ', d.status::text) AS detail,
                    d.created_at AS occurred_at
                FROM disputes d
                INNER JOIN orders o ON o.id = d.order_id

                UNION ALL

                SELECT
                    'EVENT'::text AS kind,
                    n.title AS title,
                    n.message AS detail,
                    n.created_at AS occurred_at
                FROM notifications n
                WHERE n.type::text IN ('BID_PLACED', 'WINNER_DETERMINED', 'ORDER_UPDATE', 'SYSTEM')
            ) AS events
            ORDER BY events.occurred_at DESC
            LIMIT 40
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AdminError::Repository("Failed to load activity events.".to_string()))?;

        Ok(SystemActivitySnapshot {
            kpi: SystemActivityKpi::from(kpi_row),
            recent_events: event_rows
                .into_iter()
                .map(SystemActivityEvent::from)
                .collect(),
        })
    }

    async fn resolve_dispute(
        &self,
        dispute_id: Uuid,
        outcome: DisputeResolutionOutcome,
        resolution: &str,
    ) -> Result<Dispute, AdminError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|_| AdminError::Repository("Failed to resolve dispute.".to_string()))?;

        let row = sqlx::query_as::<_, (Uuid, String, Uuid, Uuid, String)>(
            r#"
            SELECT
                d.order_id,
                d.status::text AS dispute_status,
                o.buyer_id,
                o.seller_id,
                o.title
            FROM disputes d
            INNER JOIN orders o ON o.id = d.order_id
            WHERE d.id = $1
            FOR UPDATE
            "#,
        )
        .bind(dispute_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| AdminError::Repository("Failed to resolve dispute.".to_string()))?;

        let Some((order_id, current_status, buyer_id, seller_id, order_title)) = row else {
            tx.rollback().await.ok();
            return Err(AdminError::NotFound("Dispute not found.".to_string()));
        };

        if matches!(
            current_status.as_str(),
            "RESOLVED_BUYER" | "RESOLVED_SELLER" | "CLOSED"
        ) {
            tx.rollback().await.ok();
            return Err(AdminError::Conflict(
                "Dispute already resolved.".to_string(),
            ));
        }

        let (next_dispute_status, next_order_status, buyer_title, seller_title) = match outcome {
            DisputeResolutionOutcome::Buyer => (
                "RESOLVED_BUYER",
                "REFUNDED",
                "Sengketa disetujui untuk Anda",
                "Sengketa diputuskan untuk buyer",
            ),
            DisputeResolutionOutcome::Seller => (
                "RESOLVED_SELLER",
                "CONFIRMED",
                "Sengketa diputuskan untuk seller",
                "Sengketa disetujui untuk Anda",
            ),
        };

        sqlx::query(
            r#"
            UPDATE disputes
            SET
                status = $2::dispute_status,
                resolution = $3,
                resolved_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(dispute_id)
        .bind(next_dispute_status)
        .bind(resolution)
        .execute(&mut *tx)
        .await
        .map_err(|_| AdminError::Repository("Failed to resolve dispute.".to_string()))?;

        sqlx::query(
            r#"
            UPDATE orders
            SET
                status = $2::order_status,
                is_disputed = FALSE,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(order_id)
        .bind(next_order_status)
        .execute(&mut *tx)
        .await
        .map_err(|_| AdminError::Repository("Failed to resolve dispute.".to_string()))?;

        let buyer_message = format!(
            "Order \"{}\" sudah diselesaikan admin. Keputusan: {}.",
            order_title, resolution
        );
        let seller_message = format!(
            "Order \"{}\" sudah diselesaikan admin. Keputusan: {}.",
            order_title, resolution
        );

        sqlx::query(
            r#"
            INSERT INTO notifications (
                user_id,
                type,
                title,
                message,
                reference_id,
                reference_type
            )
            VALUES (
                $1,
                'DISPUTE_RESOLVED'::notification_type,
                $2,
                $3,
                $4,
                'dispute'::reference_type
            )
            "#,
        )
        .bind(buyer_id)
        .bind(buyer_title)
        .bind(buyer_message)
        .bind(dispute_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| AdminError::Repository("Failed to resolve dispute.".to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO notifications (
                user_id,
                type,
                title,
                message,
                reference_id,
                reference_type
            )
            VALUES (
                $1,
                'DISPUTE_RESOLVED'::notification_type,
                $2,
                $3,
                $4,
                'dispute'::reference_type
            )
            "#,
        )
        .bind(seller_id)
        .bind(seller_title)
        .bind(seller_message)
        .bind(dispute_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| AdminError::Repository("Failed to resolve dispute.".to_string()))?;

        let updated = sqlx::query_as::<_, DisputeRow>(
            r#"
            SELECT
                d.id,
                d.order_id,
                d.opened_by,
                d.reason::text AS reason,
                d.description,
                d.status::text AS status,
                d.resolution,
                d.created_at,
                d.resolved_at,
                o.buyer_id,
                o.buyer_name,
                o.seller_id,
                o.seller_name,
                o.title AS order_title,
                o.image_url AS order_image_url,
                o.final_price,
                o.status::text AS order_status
            FROM disputes d
            INNER JOIN orders o ON o.id = d.order_id
            WHERE d.id = $1
            LIMIT 1
            "#,
        )
        .bind(dispute_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| AdminError::Repository("Failed to resolve dispute.".to_string()))?;

        tx.commit()
            .await
            .map_err(|_| AdminError::Repository("Failed to resolve dispute.".to_string()))?;

        Ok(Dispute::from(updated))
    }
}
