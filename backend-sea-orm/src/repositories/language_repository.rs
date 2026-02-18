use sea_orm::{DatabaseConnection, EntityTrait};
use entity::languages;
use crate::dto::language_dto::LanguageDto;
use crate::errors::Error;

pub async fn get_all(
    db: &DatabaseConnection
)->Result<Vec<LanguageDto>,Error>{
    let languages = languages::Entity::find().all(db).await?;
    
    Ok(languages.into_iter().map(LanguageDto::from).collect())
}