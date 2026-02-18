use sea_orm::DatabaseConnection;
use crate::dto::language_dto::LanguageDto;
use crate::errors::Error;
use crate::repositories::language_repository;

pub async fn get_all(
    db: &DatabaseConnection
)->Result<Vec<LanguageDto>,Error>{
    let languages = language_repository::get_all(&db).await?;
    Ok(languages)
}