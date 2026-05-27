use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AdminCategory {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub image_url: Option<String>,
    pub child_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
