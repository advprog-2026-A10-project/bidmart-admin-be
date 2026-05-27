use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SystemActivityKpi {
    pub active_auctions: i64,
    pub bids_last_24h: i64,
    pub open_disputes: i64,
    pub published_events_last_24h: i64,
}

#[derive(Debug, Clone)]
pub struct SystemActivityEvent {
    pub kind: String,
    pub title: String,
    pub detail: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SystemActivitySnapshot {
    pub kpi: SystemActivityKpi,
    pub recent_events: Vec<SystemActivityEvent>,
}
