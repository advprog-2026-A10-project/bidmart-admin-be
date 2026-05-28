use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::modules::admin::domain::entities::{
    SystemActivityEvent, SystemActivityKpi, SystemActivitySnapshot,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemActivityKpiDto {
    pub active_auctions: i64,
    pub bids_last_24h: i64,
    pub open_disputes: i64,
    pub published_events_last_24h: i64,
}

impl From<SystemActivityKpi> for SystemActivityKpiDto {
    fn from(value: SystemActivityKpi) -> Self {
        Self {
            active_auctions: value.active_auctions,
            bids_last_24h: value.bids_last_24h,
            open_disputes: value.open_disputes,
            published_events_last_24h: value.published_events_last_24h,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemActivityEventDto {
    pub kind: String,
    pub title: String,
    pub detail: String,
    pub occurred_at: DateTime<Utc>,
}

impl From<SystemActivityEvent> for SystemActivityEventDto {
    fn from(value: SystemActivityEvent) -> Self {
        Self {
            kind: value.kind,
            title: value.title,
            detail: value.detail,
            occurred_at: value.occurred_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemActivitySnapshotDto {
    pub generated_at: DateTime<Utc>,
    pub kpi: SystemActivityKpiDto,
    pub recent_events: Vec<SystemActivityEventDto>,
}

impl From<SystemActivitySnapshot> for SystemActivitySnapshotDto {
    fn from(value: SystemActivitySnapshot) -> Self {
        Self {
            generated_at: Utc::now(),
            kpi: SystemActivityKpiDto::from(value.kpi),
            recent_events: value
                .recent_events
                .into_iter()
                .map(SystemActivityEventDto::from)
                .collect(),
        }
    }
}
