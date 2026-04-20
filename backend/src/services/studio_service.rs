use chrono::DateTime;
use crate::dto::studio_dto::{DashboardStats, RecipeAnalytics, RecipeTelemetry};
use sea_orm::DatabaseConnection;
use serde_json::json;
use uuid::Uuid;
use crate::errors::Error;
use crate::repositories::recipe_repository;
use crate::services::recipe_service;

pub async fn get_stats(
    db: &DatabaseConnection,
    user_id: Uuid
) -> Result<DashboardStats, Error> {
    let (total, public, views) = recipe_repository::get_dashboard_stats(db, user_id).await?;

    Ok(DashboardStats {
        total_recipes: total as i32,
        public_recipes: public as i32,
        private_recipes: (total - public) as i32,
        total_views: views as i32,
    })
}

pub async fn get_recipe_telemetry(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    user_id: Uuid,
) -> Result<RecipeAnalytics, Error> {
    let recipe = recipe_repository::find_by_id(db, recipe_id).await?;
    if recipe.author_id != Some(user_id) {
        return Err(Error::Forbidden(json!({"message": "Unauthorized"})));
    }

    let total_views = recipe_repository::get_total_views(db, recipe_id).await?;
    let views_7d = recipe_repository::get_views_last_7_days(db, recipe_id).await?;

    let (steps, ingredients) = recipe_repository::get_recipe_counts(db, recipe_id)
        .await?
        .unwrap_or((0, 0));

    let mut health_score = 50.0;
    if recipe.prep_time_minutes > 0 { health_score += 10.0; }
    if recipe.cook_time_minutes > 0 { health_score += 10.0; }

    if steps > 0 && ingredients > 0 {
        let ratio = ingredients as f32 / steps as f32;
        if ratio >= 0.5 && ratio <= 2.0 { health_score += 20.0; }
    }

    Ok(RecipeAnalytics {
        total_views,
        views_7d,
        avg_session_duration: "02:45".to_string(),
        print_count: 12,
        share_count: 5,
        last_modified: DateTime::from(recipe.updated_at.unwrap_or_default()),
        health_score,
    })
}