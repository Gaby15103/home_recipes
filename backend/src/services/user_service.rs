use std::ops::Deref;
use crate::dto::recipe_dto::RecipeViewDto;
use crate::dto::session_dto::SessionResponseDto;
use crate::dto::user_dto::{UpdatePasswordDto, UpdateUserDto, UserResponseDto};
use crate::errors::Error;
use crate::repositories::{
    recipe_repository, recipe_translation_repository, role_repository, session_repository,
    user_repository,
};
use crate::utils::HASHER;
use entity::prelude::Users;
use entity::{recipes, roles, users};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde_json::json;
use uuid::Uuid;

pub async fn get_active_sessions(
    db: &DatabaseConnection,
    user_id: Uuid,
    current_session_id: Uuid,
) -> Result<Vec<SessionResponseDto>, Error> {
    let sessions = session_repository::get_user_sessions(db, user_id).await?;

    // We pass the current_session_id if we want to flag "is_current" in the DTO
    Ok(sessions
        .into_iter()
        .map(|m| SessionResponseDto::from_model(m, current_session_id))
        .collect())
}

pub async fn revoke_session(
    db: &DatabaseConnection,
    user_id: Uuid,
    session_id: Uuid,
) -> Result<(), Error> {
    let session = session_repository::find_by_id(db, session_id)
        .await?
        .ok_or(Error::NotFound(json!({"error": "Session not found"})))?;

    if session.user_id != user_id {
        return Err(Error::Unauthorized("You do not own this session".into()));
    }

    session_repository::delete_session_by_id(db, session_id).await?;
    Ok(())
}

pub async fn update_user(
    db: &DatabaseConnection,
    user_id: Uuid,
    data: UpdateUserDto,
) -> Result<users::Model, Error> {
    // Just delegate to repository
    user_repository::update_user_profile(db, user_id, data).await
}

pub async fn change_password(
    db: &DatabaseConnection,
    user_id: Uuid,
    data: UpdatePasswordDto,
) -> Result<(), Error> {
    // 1. Verify credentials (Business logic)
    let user = user_repository::find_by_id(db, user_id).await?;
    crate::utils::password_verification::verify_password(&user.password_hash, &data.old_password)?;

    let new_hash = HASHER.hash(&data.new_password)?;
    user_repository::reset_password(db, user_id, new_hash).await
}

pub async fn get_favorites(
    db: &DatabaseConnection,
    user: UserResponseDto,
) -> Result<Vec<RecipeViewDto>, Error> {
    let recipes = recipe_repository::get_favorites(db, user.id.clone()).await?;

    let mut dtos = Vec::new();

    for recipe in recipes {
        let translation = recipe_translation_repository::find_translation(
            db,
            recipe.id,
            user.preferences.language.deref(),
            recipe.original_language_code.deref(),
        )
        .await?;
        let dto = RecipeViewDto::from((recipe, translation));
        dtos.push(dto);
    }

    Ok(dtos)
}
