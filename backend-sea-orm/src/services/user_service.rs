use sea_orm::{DatabaseConnection, Set, ActiveModelTrait};
use uuid::Uuid;
use crate::dto::user_dto::{UserResponseDto, UpdateUserDto, UpdatePasswordDto};
use crate::repositories::{user_repository, session_repository, role_repository};
use crate::errors::Error;
use entity::{users, roles};
use serde_json::json;
use crate::dto::session_dto::SessionResponseDto;
use crate::utils::HASHER;

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

    let session = session_repository::find_by_id(db, session_id).await?
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
