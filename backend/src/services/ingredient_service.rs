use sea_orm::DatabaseConnection;
use std::ops::Deref;
use uuid::Uuid;
use crate::dto::ingredient_dto::{IngredientRecipeViewDto, IngredientViewDto};
use crate::errors::Error;
use crate::repositories::ingredient_repository;

pub async fn get_all(
    db: &DatabaseConnection,
    search: Option<String>,
    limit: i32,
    lang_code: &str,
)->Result<Vec<IngredientViewDto>, Error>{
    let ingredients: Vec<IngredientViewDto> = ingredient_repository::get_all(db, search, limit, lang_code).await?;
    Ok(ingredients)
}