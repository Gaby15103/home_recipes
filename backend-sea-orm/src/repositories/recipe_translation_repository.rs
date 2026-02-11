use sea_orm::{ColumnTrait, DbErr};
use sea_orm::{EntityTrait, QueryFilter};
use sea_orm::{DatabaseConnection};
use uuid::Uuid;
use entity::{recipe_translations, recipes};
use crate::errors::Error;

pub async fn find_all(
    db: &DatabaseConnection
) -> Result<Vec<recipe_translations::Model>, Error> {

    recipe_translations::Entity::find().all(db).await.map_err(Error::from)
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid
) -> Result<recipe_translations::Model, DbErr> {
    recipe_translations::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Recipe not found".into()))
}
pub async fn find_by_recipe_and_lang(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    requested_lang: &str,
    fallback_lang: &str,
)-> Result<recipe_translations::Model, Error> {
    let translation = recipe_translations::Entity::find()
        .filter(recipe_translations::Column::RecipeId.eq(recipe_id))
        .filter(recipe_translations::Column::LanguageCode.eq(requested_lang))
        .one(db)
        .await?;

    if let Some(t) = translation {
        return Ok(t);
    }

    let fallback = recipe_translations::Entity::find()
        .filter(recipe_translations::Column::RecipeId.eq(recipe_id))
        .filter(recipe_translations::Column::LanguageCode.eq(fallback_lang))
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound(serde_json::json!({
            "error": "Translation not found"
        })))?;

    Ok(fallback)
}