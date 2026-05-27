use std::sync::Arc;

use crate::modules::admin::application::dto::CategoryDto;
use crate::modules::admin::domain::errors::AdminError;
use crate::modules::admin::domain::traits::CoreAdminRepository;

pub struct CategoryUseCase {
    core_repo: Arc<dyn CoreAdminRepository>,
}

impl CategoryUseCase {
    pub fn new(core_repo: Arc<dyn CoreAdminRepository>) -> Self {
        Self { core_repo }
    }

    pub async fn list_categories(&self) -> Result<Vec<CategoryDto>, AdminError> {
        let categories = self.core_repo.list_categories().await?;
        Ok(categories.into_iter().map(CategoryDto::from).collect())
    }

    pub async fn create_category(
        &self,
        name: &str,
        slug: &str,
        parent_id: Option<i32>,
        image_url: Option<String>,
    ) -> Result<CategoryDto, AdminError> {
        let normalized_name = normalize_name(name)?;
        let normalized_slug = normalize_slug(slug)?;
        let normalized_parent = normalize_parent(parent_id)?;
        let normalized_image = normalize_image(image_url);

        let category = self
            .core_repo
            .create_category(
                &normalized_name,
                &normalized_slug,
                normalized_parent,
                normalized_image,
            )
            .await?;
        Ok(CategoryDto::from(category))
    }

    pub async fn update_category(
        &self,
        category_id: i32,
        name: &str,
        slug: &str,
        parent_id: Option<i32>,
        image_url: Option<String>,
    ) -> Result<CategoryDto, AdminError> {
        if category_id <= 0 {
            return Err(AdminError::InvalidInput("Invalid category id.".to_string()));
        }

        let normalized_name = normalize_name(name)?;
        let normalized_slug = normalize_slug(slug)?;
        let normalized_parent = normalize_parent(parent_id)?;
        let normalized_image = normalize_image(image_url);

        let category = self
            .core_repo
            .update_category(
                category_id,
                &normalized_name,
                &normalized_slug,
                normalized_parent,
                normalized_image,
            )
            .await?;
        Ok(CategoryDto::from(category))
    }

    pub async fn delete_category(&self, category_id: i32) -> Result<(), AdminError> {
        if category_id <= 0 {
            return Err(AdminError::InvalidInput("Invalid category id.".to_string()));
        }
        self.core_repo.delete_category(category_id).await
    }
}

fn normalize_name(value: &str) -> Result<String, AdminError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AdminError::InvalidInput("Name is required.".to_string()));
    }
    Ok(trimmed.to_string())
}

fn normalize_slug(value: &str) -> Result<String, AdminError> {
    let trimmed = value.trim().to_ascii_lowercase();
    if trimmed.is_empty() {
        return Err(AdminError::InvalidInput("Slug is required.".to_string()));
    }
    if !trimmed
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        return Err(AdminError::InvalidInput(
            "Slug can only contain lowercase letters, digits, and '-'.".to_string(),
        ));
    }
    if trimmed.starts_with('-') || trimmed.ends_with('-') || trimmed.contains("--") {
        return Err(AdminError::InvalidInput(
            "Slug format is invalid.".to_string(),
        ));
    }
    Ok(trimmed)
}

fn normalize_parent(parent_id: Option<i32>) -> Result<Option<i32>, AdminError> {
    match parent_id {
        Some(value) if value <= 0 => Err(AdminError::InvalidInput(
            "Parent category id must be positive.".to_string(),
        )),
        other => Ok(other),
    }
}

fn normalize_image(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}
