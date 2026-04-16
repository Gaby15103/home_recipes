use crate::dto::studio_dto::DashboardStats;
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::errors::Error;
use crate::repositories::recipe_repository;

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
