use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct AdminDashboardSummaryRow {
    pub total_users: i64,
    pub active_users: i64,
    pub disabled_users: i64,
    pub pending_users: i64,
    pub active_sessions: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ManagedUserRow {
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

#[derive(Debug, Clone, FromRow)]
pub struct ManagedUserSessionRow {
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
}

#[derive(Debug, Clone, FromRow)]
pub struct ModerationListingRow {
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

#[derive(Debug, Clone, FromRow)]
pub struct DisputeRow {
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
pub enum ResolveDisputeOutcome {
    Buyer,
    Seller,
}

#[derive(Debug)]
pub enum ResolveDisputeError {
    NotFound,
    AlreadyResolved,
    Database(sqlx::Error),
}

impl From<sqlx::Error> for ResolveDisputeError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}

pub async fn get_dashboard_summary(pool: &PgPool) -> Result<AdminDashboardSummaryRow, sqlx::Error> {
    sqlx::query_as::<_, AdminDashboardSummaryRow>(
        r#"
        SELECT
            COUNT(*)::bigint AS total_users,
            COUNT(*) FILTER (WHERE u.status = 'ACTIVE')::bigint AS active_users,
            COUNT(*) FILTER (WHERE u.status = 'DISABLED')::bigint AS disabled_users,
            COUNT(*) FILTER (WHERE u.status = 'PENDING_VERIFICATION')::bigint AS pending_users,
            (
                SELECT COUNT(*)::bigint
                FROM sessions s
                WHERE s.expired_at > NOW()
            ) AS active_sessions
        FROM users u
        "#,
    )
    .fetch_one(pool)
    .await
}

pub async fn list_managed_users(pool: &PgPool) -> Result<Vec<ManagedUserRow>, sqlx::Error> {
    sqlx::query_as::<_, ManagedUserRow>(
        r#"
        SELECT
            u.id,
            COALESCE(
                NULLIF(TRIM(CONCAT(COALESCE(p.first_name, ''), ' ', COALESCE(p.last_name, ''))), ''),
                split_part(u.email, '@', 1)
            ) AS name,
            u.email,
            u.status::text AS status,
            COALESCE(
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT r.name), NULL),
                ARRAY[]::text[]
            ) AS roles,
            u.created_at,
            MAX(s.last_active_at) AS last_seen_at,
            COUNT(*) FILTER (WHERE s.expired_at > NOW())::bigint AS active_sessions,
            (u.email_verified_at IS NOT NULL) AS email_verified,
            u.mfa_email_enabled,
            u.mfa_totp_enabled
        FROM users u
        LEFT JOIN user_profiles p ON p.user_id = u.id
        LEFT JOIN user_roles ur ON ur.user_id = u.id
        LEFT JOIN roles r ON r.id = ur.role_id
        LEFT JOIN sessions s ON s.user_id = u.id
        GROUP BY
            u.id, p.first_name, p.last_name, u.email, u.status, u.created_at,
            u.email_verified_at, u.mfa_email_enabled, u.mfa_totp_enabled
        ORDER BY u.created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_managed_user_by_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<ManagedUserRow>, sqlx::Error> {
    sqlx::query_as::<_, ManagedUserRow>(
        r#"
        SELECT
            u.id,
            COALESCE(
                NULLIF(TRIM(CONCAT(COALESCE(p.first_name, ''), ' ', COALESCE(p.last_name, ''))), ''),
                split_part(u.email, '@', 1)
            ) AS name,
            u.email,
            u.status::text AS status,
            COALESCE(
                ARRAY_REMOVE(ARRAY_AGG(DISTINCT r.name), NULL),
                ARRAY[]::text[]
            ) AS roles,
            u.created_at,
            MAX(s.last_active_at) AS last_seen_at,
            COUNT(*) FILTER (WHERE s.expired_at > NOW())::bigint AS active_sessions,
            (u.email_verified_at IS NOT NULL) AS email_verified,
            u.mfa_email_enabled,
            u.mfa_totp_enabled
        FROM users u
        LEFT JOIN user_profiles p ON p.user_id = u.id
        LEFT JOIN user_roles ur ON ur.user_id = u.id
        LEFT JOIN roles r ON r.id = ur.role_id
        LEFT JOIN sessions s ON s.user_id = u.id
        WHERE u.id = $1
        GROUP BY
            u.id, p.first_name, p.last_name, u.email, u.status, u.created_at,
            u.email_verified_at, u.mfa_email_enabled, u.mfa_totp_enabled
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_user_sessions(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<ManagedUserSessionRow>, sqlx::Error> {
    sqlx::query_as::<_, ManagedUserSessionRow>(
        r#"
        SELECT
            s.id,
            s.device,
            s.browser,
            s.os,
            s.ip,
            s.location,
            s.mfa_satisfied,
            s.created_at,
            s.last_active_at,
            s.expired_at
        FROM sessions s
        WHERE s.user_id = $1
        ORDER BY s.last_active_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn list_moderation_listings(
    pool: &PgPool,
) -> Result<Vec<ModerationListingRow>, sqlx::Error> {
    sqlx::query_as::<_, ModerationListingRow>(
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
    .fetch_all(pool)
    .await
}

pub async fn find_moderation_listing_by_id(
    pool: &PgPool,
    listing_id: Uuid,
) -> Result<Option<ModerationListingRow>, sqlx::Error> {
    sqlx::query_as::<_, ModerationListingRow>(
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
    .fetch_optional(pool)
    .await
}

pub async fn list_disputes(pool: &PgPool) -> Result<Vec<DisputeRow>, sqlx::Error> {
    sqlx::query_as::<_, DisputeRow>(
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
    .fetch_all(pool)
    .await
}

pub async fn find_dispute_by_id(
    pool: &PgPool,
    dispute_id: Uuid,
) -> Result<Option<DisputeRow>, sqlx::Error> {
    sqlx::query_as::<_, DisputeRow>(
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
    .fetch_optional(pool)
    .await
}

pub async fn resolve_dispute(
    pool: &PgPool,
    dispute_id: Uuid,
    outcome: ResolveDisputeOutcome,
    resolution: &str,
) -> Result<DisputeRow, ResolveDisputeError> {
    let mut tx = pool.begin().await.map_err(ResolveDisputeError::Database)?;

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
    .map_err(ResolveDisputeError::Database)?;

    let Some((order_id, current_status, buyer_id, seller_id, order_title)) = row else {
        tx.rollback().await.ok();
        return Err(ResolveDisputeError::NotFound);
    };

    if matches!(
        current_status.as_str(),
        "RESOLVED_BUYER" | "RESOLVED_SELLER" | "CLOSED"
    ) {
        tx.rollback().await.ok();
        return Err(ResolveDisputeError::AlreadyResolved);
    }

    let (next_dispute_status, next_order_status, buyer_title, seller_title) = match outcome {
        ResolveDisputeOutcome::Buyer => (
            "RESOLVED_BUYER",
            "REFUNDED",
            "Sengketa disetujui untuk Anda",
            "Sengketa diputuskan untuk buyer",
        ),
        ResolveDisputeOutcome::Seller => (
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
    .map_err(ResolveDisputeError::Database)?;

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
    .map_err(ResolveDisputeError::Database)?;

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
    .map_err(ResolveDisputeError::Database)?;

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
    .map_err(ResolveDisputeError::Database)?;

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
    .map_err(ResolveDisputeError::Database)?;

    tx.commit().await.map_err(ResolveDisputeError::Database)?;

    Ok(updated)
}
