use crate::dto::recipe_dto::RecipeEditorDto;
use crate::dto::recipe_version_dto::RecipeVersionDto;
use crate::dto::user_dto::UserResponseDto;
use crate::errors::Error;
use crate::repositories::role_repository;
use chrono::Utc;
use entity::{recipe_versions, users};
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm::{QueryFilter, QueryOrder};
use serde_json::json;
use uuid::Uuid;

pub async fn create(
    db: &DatabaseConnection,
    recipe: RecipeEditorDto,
    user_id: Uuid,
) -> Result<(), Error> {
    let version_data = serde_json::to_value(&recipe)
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to serialize recipe data for version",
            "operation": "create",
            "entity": "recipe_versions",
            "recipe_id": recipe.id.to_string(),
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "serialization"
        })))?;

    recipe_versions::ActiveModel {
        id: Default::default(),
        recipe_id: Set(recipe.id),
        data: Set(version_data),
        edited_by: Set(Some(user_id)),
        ..Default::default()
    }
        .insert(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to insert recipe version",
        "operation": "create",
        "entity": "recipe_versions",
        "recipe_id": recipe.id.to_string(),
        "user_id": user_id.to_string(),
        "error": e.to_string(),
        "stage": "insert"
    })))?;

    Ok(())
}

pub async fn get_versions(
    db: &DatabaseConnection,
    recipe_id: Uuid,
) -> Result<Vec<RecipeVersionDto>, Error> {
    let res = recipe_versions::Entity::find()
        .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
        .find_also_related(users::Entity)
        .order_by_desc(recipe_versions::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch recipe versions",
            "operation": "get_versions",
            "entity": "recipe_versions",
            "recipe_id": recipe_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?;

    let mut dtos = Vec::new();

    for (idx, (version, user_opt)) in res.iter().enumerate() {
        let recipe_data: RecipeEditorDto = serde_json::from_value(version.data.clone())
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to deserialize recipe version data",
                "operation": "get_versions",
                "entity": "recipe_versions",
                "recipe_id": recipe_id.to_string(),
                "version_id": version.id.to_string(),
                "version_index": idx,
                "error": e.to_string(),
                "stage": "deserialization"
            })))?;

        if let Some(user_model) = user_opt {
            let roles = role_repository::get_roles_for_user(db, user_model.id).await?;

            let user_dto = UserResponseDto::from((user_model.clone(), roles));

            dtos.push(RecipeVersionDto {
                id: version.id,
                data: recipe_data,
                recipe_id: version.recipe_id,
                edited_by: user_dto,
                created_at: version.created_at.with_timezone(&Utc),
            });
        }
    }

    Ok(dtos)
}

pub async fn get_version(
    db: &DatabaseConnection,
    recipe_id: Uuid,
    version_id: Uuid,
) -> Result<RecipeVersionDto, Error> {
    let res = recipe_versions::Entity::find_by_id(version_id)
        .filter(recipe_versions::Column::RecipeId.eq(recipe_id))
        .find_also_related(users::Entity)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch recipe version",
            "operation": "get_version",
            "entity": "recipe_versions",
            "recipe_id": recipe_id.to_string(),
            "version_id": version_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?;

    if let Some((version, user_opt)) = res {
        let recipe_data: RecipeEditorDto = serde_json::from_value(version.data)
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to deserialize recipe version data",
                "operation": "get_version",
                "entity": "recipe_versions",
                "recipe_id": recipe_id.to_string(),
                "version_id": version_id.to_string(),
                "error": e.to_string(),
                "stage": "deserialization"
            })))?;

        let user_model = user_opt
            .ok_or_else(|| Error::InternalServerError(json!({
                "message": "Editor information missing for recipe version",
                "operation": "get_version",
                "entity": "recipe_versions",
                "recipe_id": recipe_id.to_string(),
                "version_id": version_id.to_string(),
                "stage": "validation"
            })))?;

        let roles = role_repository::get_roles_for_user(db, user_model.id).await?;
        let user_dto = UserResponseDto::from((user_model, roles));

        Ok(RecipeVersionDto {
            id: version.id,
            data: recipe_data,
            recipe_id: version.recipe_id,
            edited_by: user_dto,
            created_at: version.created_at.with_timezone(&Utc),
        })
    } else {
        Err(Error::InternalServerError(json!({
            "message": "Recipe version not found",
            "operation": "get_version",
            "entity": "recipe_versions",
            "recipe_id": recipe_id.to_string(),
            "version_id": version_id.to_string(),
            "stage": "validation"
        })))
    }
}