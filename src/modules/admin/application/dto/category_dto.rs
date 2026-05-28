use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::admin::domain::entities::AdminCategory;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDto {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub image_url: Option<String>,
    pub child_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AdminCategory> for CategoryDto {
    fn from(value: AdminCategory) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            name: value.name,
            slug: value.slug,
            image_url: value.image_url,
            child_count: value.child_count,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryCommand {
    pub name: String,
    pub slug: String,
    pub parent_id: Option<i32>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryCommand {
    pub name: String,
    pub slug: String,
    pub parent_id: Option<i32>,
    pub image_url: Option<String>,
}
