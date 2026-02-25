use crate::errors::Error;
use entity::{roles, users};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use serde_json::json;
use uuid::Uuid;

pub async fn get_roles_for_user(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<roles::Model>, Error> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({"error": "User not found"})))?;

    let roles: Vec<roles::Model> = user.find_related(roles::Entity).all(db).await?;

    Ok(roles)
}
