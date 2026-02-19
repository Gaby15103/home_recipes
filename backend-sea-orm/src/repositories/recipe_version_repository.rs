use crate::dto::recipe_dto::RecipeEditorDto;
use crate::errors::Error;
use entity::recipe_versions;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

pub async fn create(
    db: &DatabaseConnection,
    recipe: RecipeEditorDto,
    user_id: Uuid,
) -> Result<(), Error> {
    let version_data = serde_json::to_value(&recipe)?;
    recipe_versions::ActiveModel {
        id: Default::default(),
        recipe_id: Set(recipe.id),
        data: Set(version_data),
        edited_by: Set(Some(user_id)),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(())
}
