#[derive(Debug, Clone)]
pub struct AdminDashboardSummary {
    pub total_users: i64,
    pub active_users: i64,
    pub disabled_users: i64,
    pub pending_users: i64,
    pub active_sessions: i64,
}
