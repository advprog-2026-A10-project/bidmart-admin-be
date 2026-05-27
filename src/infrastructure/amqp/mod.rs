use std::time::Duration;

use futures_util::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    Connection, ConnectionProperties, ExchangeKind,
};
use serde::Deserialize;

use crate::modules::auth::infrastructure::middleware::{
    invalidate_cache_for_role, invalidate_cache_for_user, AuthzCache,
};

const USER_ROLE_KEY: &str = "auth.user_role_changed";
const ROLE_PERMISSION_KEY: &str = "auth.role_permissions_changed";
const CONSUMER_TAG: &str = "admin-authz-cache-invalidator";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventEnvelope {
    event_type: Option<String>,
    data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserRoleChangedData {
    user_id: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RolePermissionsChangedData {
    role: String,
}

async fn run_consumer_once(
    cache: &AuthzCache,
    amqp_url: &str,
    exchange: &str,
    queue_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let connection = Connection::connect(amqp_url, ConnectionProperties::default()).await?;
    let channel = connection.create_channel().await?;

    channel
        .exchange_declare(
            exchange,
            ExchangeKind::Topic,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let queue = channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            queue.name().as_str(),
            exchange,
            USER_ROLE_KEY,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            queue.name().as_str(),
            exchange,
            ROLE_PERMISSION_KEY,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    tracing::info!(exchange, queue = %queue.name(), "admin authz invalidation consumer ready");

    let mut consumer = channel
        .basic_consume(
            queue.name().as_str(),
            CONSUMER_TAG,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        let routing_key = delivery.routing_key.clone();

        match serde_json::from_slice::<EventEnvelope>(&delivery.data) {
            Ok(event) => {
                let event_type = event.event_type.unwrap_or_else(|| routing_key.to_string());
                match event_type.as_str() {
                    USER_ROLE_KEY => {
                        if let Ok(data) = serde_json::from_value::<UserRoleChangedData>(event.data)
                        {
                            let removed = invalidate_cache_for_user(cache, data.user_id).await;
                            tracing::info!(user_id = %data.user_id, removed, "authz cache invalidated by user role event");
                        }
                    }
                    ROLE_PERMISSION_KEY => {
                        if let Ok(data) =
                            serde_json::from_value::<RolePermissionsChangedData>(event.data)
                        {
                            let removed = invalidate_cache_for_role(cache, &data.role).await;
                            tracing::info!(role = %data.role, removed, "authz cache invalidated by role permission event");
                        }
                    }
                    _ => {}
                }
            }
            Err(error) => {
                tracing::warn!(?error, routing_key = %routing_key, "authz invalidation ignored malformed event");
            }
        }

        if let Err(error) = delivery.ack(BasicAckOptions::default()).await {
            tracing::warn!(?error, "authz invalidation ack failed");
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::UnexpectedEof,
        "authz invalidation AMQP stream ended",
    )
    .into())
}

fn reconnect_delay(attempt: u32) -> Duration {
    let delay_ms = 500_u64.saturating_mul(2_u64.saturating_pow(attempt.min(6)));
    Duration::from_millis(delay_ms.min(30_000))
}

pub fn spawn_authz_cache_invalidation_consumer(
    cache: AuthzCache,
    amqp_url: Option<String>,
    exchange: String,
    queue_name: String,
) {
    let Some(amqp_url) = amqp_url.filter(|value| !value.trim().is_empty()) else {
        tracing::info!("authz invalidation consumer skipped: APP_AMQP_URL is not configured");
        return;
    };

    tokio::spawn(async move {
        let mut attempt = 0_u32;

        loop {
            match run_consumer_once(&cache, &amqp_url, &exchange, &queue_name).await {
                Ok(()) => {
                    attempt = 0;
                }
                Err(error) => {
                    attempt = attempt.saturating_add(1);
                    let delay = reconnect_delay(attempt);
                    tracing::warn!(
                        ?error,
                        attempt,
                        delay_ms = delay.as_millis(),
                        "authz invalidation consumer failed, reconnecting"
                    );
                    tokio::time::sleep(delay).await;
                }
            }
        }
    });
}
