use crate::dto::recipe_dto::CreateRecipeInput;
use crate::errors::Error;
use crate::recipe_parser;
use crate::recipe_parser::ParserContext;
use crate::repositories::unit_repository;
use actix_multipart::form::tempfile::TempFile;
use sea_orm::DatabaseConnection;
use sqlx::SqlitePool;

pub async fn recipe_from_file(
    image: TempFile,
    db: &DatabaseConnection,
    sqlite_pool: &SqlitePool,
) -> Result<CreateRecipeInput, Error> {
    let units = unit_repository::get_all_admin(db).await?;

    let context = ParserContext {
        sqlite_pool: &sqlite_pool,
        known_units: units,
    };

    let recipe = recipe_parser::run_pipeline(image.file.path(), context).await?;

    Ok(recipe)
}
