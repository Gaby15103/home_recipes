use crate::dto::tag_dto::{InputTag, TagDto};
use crate::errors::Error;
use entity::{recipe_tags, tags};
use migration::JoinType;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, EntityTrait, Set};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, QuerySelect, RelationTrait};
use uuid::Uuid;

pub async fn find_or_create_tags(
    txn: &DatabaseTransaction,
    tags_input: Vec<InputTag>,
    recipe_id: Uuid,
) -> Result<Vec<TagDto>, Error> {
    let mut tags = Vec::new();
    for tag_input in tags_input {
        let tag_dto = find_or_create_tag(txn, tag_input, recipe_id).await?;
        tags.push(tag_dto);
    }
    Ok(tags)
}

pub async fn find_or_create_tag(
    txn: &DatabaseTransaction,
    tag_input: InputTag,
    recipe_id: Uuid,
) -> Result<TagDto, Error> {
    let tag_model = match tag_input {
        InputTag::Existing { id } => tags::Entity::find_by_id(id)
            .one(txn)
            .await?
            .ok_or_else(|| Error::NotFound(serde_json::json!({"error": "Tag not found"})))?,

        InputTag::New { name } => {
            let normalized_name = name.trim().to_lowercase();

            let existing = tags::Entity::find()
                .filter(tags::Column::Name.eq(&normalized_name))
                .one(txn)
                .await?;

            if let Some(t) = existing {
                t
            } else {
                tags::ActiveModel {
                    name: Set(normalized_name),
                    ..Default::default()
                }
                .insert(txn)
                .await?
            }
        }
    };

    let exists = recipe_tags::Entity::find()
        .filter(recipe_tags::Column::RecipeId.eq(recipe_id))
        .filter(recipe_tags::Column::TagId.eq(tag_model.id))
        .one(txn)
        .await?;
    if exists.is_none() {
        let _ = recipe_tags::ActiveModel {
            recipe_id: Set(recipe_id),
            tag_id: Set(tag_model.id),
        }
        .insert(txn)
        .await?;
    }

    Ok(TagDto {
        id: tag_model.id,
        name: tag_model.name,
    })
}
pub async fn find_by_recipe(
    db: &DatabaseConnection,
    recipe_id: Uuid,
) -> Result<Vec<TagDto>, DbErr> {
    let tags = tags::Entity::find()
        .join(JoinType::InnerJoin, tags::Relation::RecipeTags.def())
        .filter(recipe_tags::Column::RecipeId.eq(recipe_id))
        .all(db)
        .await?;
    let mut tags_dto: Vec<TagDto> = Vec::new();
    for tag in tags {
        tags_dto.push(TagDto {
            id: tag.id,
            name: tag.name,
        })
    }
    Ok(tags_dto)
}
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<TagDto>, Error> {
    let tags = tags::Entity::find().all(db).await?;
    let tags_dto: Vec<TagDto> = tags.into_iter().map(TagDto::from).collect();
    Ok(tags_dto)
}
pub async fn create(db: &DatabaseConnection, new_tag: InputTag) -> Result<TagDto, Error> {
    let tag_model = match new_tag {
        InputTag::New { name } => {
            tags::ActiveModel {
                id: Set(Uuid::new_v4()),
                name: Set(name),
                ..Default::default()
            }
            .insert(db)
            .await?
        }
        _ => {
            return Err(Error::BadRequest(
                serde_json::json!({"error": "Expected a new tag name"}),
            ));
        }
    };

    Ok(TagDto::from(tag_model))
}
pub async fn update(db: &DatabaseConnection, updated_tag: TagDto) -> Result<TagDto, Error> {
    let existing = tags::Entity::find_by_id(updated_tag.id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(
            serde_json::json!({"error": "Tag not found"}),
        ))?;

    if existing.name == updated_tag.name {
        return Ok(TagDto::from(existing));
    }

    let mut tag: tags::ActiveModel = existing.into();
    tag.name = Set(updated_tag.name);

    let updated_model = tag.update(db).await?;

    Ok(TagDto::from(updated_model))
}
