use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, EntityTrait, Set};
use uuid::Uuid;
use entity::{recipe_tags, tags};
use crate::dto::tag_dto::{InputTag, TagDto};
use crate::errors::Error;

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
        // Case 1: Existing - We need to fetch it to get the name for the DTO
        InputTag::Existing { id } => {
            tags::Entity::find_by_id(id)
                .one(txn)
                .await?
                .ok_or_else(|| Error::NotFound(serde_json::json!({"error": "Tag not found"})))?
        }

        // Case 2: New - Find by name or create
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
                    id: Set(Uuid::new_v4()),
                    name: Set(normalized_name),
                    ..Default::default()
                }
                    .insert(txn)
                    .await?
            }
        }
    };

    // Link to the recipe in the junction table
    // We use if_not_exists logic implicitly or handle conflict if necessary
    let _ = recipe_tags::ActiveModel {
        recipe_id: Set(recipe_id),
        tag_id: Set(tag_model.id),
    }
        .insert(txn)
        .await?;

    Ok(TagDto {
        id: tag_model.id,
        name: tag_model.name,
    })
}