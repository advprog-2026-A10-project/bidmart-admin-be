use serde::Serialize;

use crate::modules::admin::domain::entities::AdminDashboardSummary;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminDashboardSummaryDto {
    pub total_users: i64,
    pub active_users: i64,
    pub disabled_users: i64,
    pub pending_users: i64,
    pub active_sessions: i64,
}

impl From<AdminDashboardSummary> for AdminDashboardSummaryDto {
    fn from(value: AdminDashboardSummary) -> Self {
        Self {
            total_users: value.total_users,
            active_users: value.active_users,
            disabled_users: value.disabled_users,
            pending_users: value.pending_users,
            active_sessions: value.active_sessions,
        }
    }
}
