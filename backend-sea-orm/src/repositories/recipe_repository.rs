use sea_orm::{ColumnTrait, DbErr};
use sea_orm::QueryFilter;
use sea_orm::{EntityTrait, DatabaseConnection};
use uuid::Uuid;
use crate::entities::prelude::RecipeTranslations;
use crate::entities::{recipe_translations, recipes};
use crate::entities::recipes::Model;
use crate::errors::Error;

pub async fn find_all(
    db: &DatabaseConnection
) -> Result<Vec<recipes::Model>, Error> {

    recipes::Entity::find().all(db).await.map_err(Error::from)
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid
) -> Result<recipes::Model, DbErr> {
    recipes::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Recipe not found".into()))
}