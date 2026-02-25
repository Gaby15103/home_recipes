use crate::dto::ingredient_dto::IngredientViewDto;
use crate::errors::Error;
use crate::repositories::ingredient_repository;
use sea_orm::DatabaseConnection;

pub async fn get_all(
    db: &DatabaseConnection,
    search: Option<String>,
    limit: i32,
    lang_code: &str,
)->Result<Vec<IngredientViewDto>, Error>{
    let ingredients: Vec<IngredientViewDto> = ingredient_repository::get_all(db, search, limit, lang_code).await?;
    Ok(ingredients)
}