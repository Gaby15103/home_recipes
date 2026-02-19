use sea_orm::{DatabaseConnection};
use crate::dto::tag_dto::{InputTag, TagDto};
use crate::errors::Error;
use crate::repositories::tag_repository;

pub async fn get_all(
    db: &DatabaseConnection,
)->Result<Vec<TagDto>, Error> {
    let tags = tag_repository::get_all(db).await?;
    Ok(tags)
}
pub async fn create(
    db: &DatabaseConnection,
    new_tag: InputTag,
)->Result<TagDto, Error> {
    let result = tag_repository::create(db, new_tag).await?;
    Ok(result)
}

pub async fn update(
    db: &DatabaseConnection,
    updated_tag: TagDto,
) ->Result<TagDto, Error> {
    let result = tag_repository::update(db, updated_tag).await?;
    Ok(result)
}