use crate::errors::Error;
use chrono::Utc;
use entity::{sessions, users};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

pub async fn find_valid_session_with_user(
    db: &DatabaseConnection,
    token: &str,
) -> Result<Option<(sessions::Model, users::Model)>, sea_orm::DbErr> {
    let result = sessions::Entity::find()
        .find_also_related(users::Entity)
        .filter(sessions::Column::Token.eq(token))
        .filter(sessions::Column::ExpiresAt.gt(Utc::now()))
        .filter(sessions::Column::IsRevoked.eq(false))
        .one(db)
        .await?;

    Ok(result.and_then(|(session, user_opt)| user_opt.map(|user| (session, user))))
}
pub async fn get_user_sessions(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<sessions::Model>, sea_orm::DbErr> {
    sessions::Entity::find()
        .filter(sessions::Column::UserId.eq(user_id))
        .filter(sessions::Column::IsRevoked.eq(false))
        .filter(sessions::Column::ExpiresAt.gt(Utc::now()))
        .all(db)
        .await
}
pub async fn find_by_id(
    db: &DatabaseConnection,
    session_id: Uuid,
) -> Result<Option<sessions::Model>, sea_orm::DbErr> {
    sessions::Entity::find_by_id(session_id).one(db).await
}
pub async fn delete_session_by_id(
    db: &DatabaseConnection,
    session_id: Uuid,
) -> Result<DeleteResult, sea_orm::DbErr> {
    sessions::Entity::delete_by_id(session_id).exec(db).await
}

pub async fn update_last_active(
    db: &DatabaseConnection,
    session: sessions::Model,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::{ActiveModelTrait, Set};

    let mut active_session: sessions::ActiveModel = session.into();

    let now: DateTimeWithTimeZone = chrono::Utc::now().into();

    active_session.last_active_at = Set(Option::from(now));
    active_session.update(db).await?;

    Ok(())
}
pub async fn create(
    db: &DatabaseConnection,
    user_id: Uuid,
    token: String,
    expires_at: DateTimeWithTimeZone,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<sessions::Model, sea_orm::DbErr> {
    let new_session = sessions::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        token: Set(token),
        user_agent: Set(user_agent),
        ip_address: Set(ip_address),
        expires_at: Set(expires_at),
        created_at: Set(Some(chrono::Utc::now().into())),
        last_active_at: Set(Some(chrono::Utc::now().into())),
        is_revoked: Set(false),
        ..Default::default()
    };

    new_session.insert(db).await
}

pub async fn delete_session_by_token(
    db: &DatabaseConnection,
    token: &str,
) -> Result<DeleteResult, Error> {
    let result = sessions::Entity::delete_many()
        .filter(sessions::Column::Token.eq(token))
        .exec(db)
        .await?;
    Ok(result)
}

pub async fn delete_all_other_sessions(
    db: &DatabaseConnection,
    user_id: i32,
    current_token: &str,
) -> Result<DeleteResult, Error> {
    let result = sessions::Entity::delete_many()
        .filter(sessions::Column::UserId.eq(user_id))
        .filter(sessions::Column::Token.ne(current_token))
        .exec(db)
        .await?;
    Ok(result)
}
pub async fn delete_all_user_sessions(db: &DatabaseConnection, user_id: Uuid) -> Result<(), Error> {
    sessions::Entity::delete_many()
        .filter(sessions::Column::UserId.eq(user_id))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn refresh_session(
    db: &DatabaseConnection,
    session_id: Uuid,
    new_token: String,
    new_expires_at: DateTimeWithTimeZone,
) -> Result<String, Error> {
    let session = sessions::Entity::find_by_id(session_id)
        .one(db)
        .await?
        .ok_or(Error::Unauthorized(
            "Session invalid or expired".to_string().parse().unwrap(),
        ))?;

    let mut session: sessions::ActiveModel = session.into();
    session.token = Set(new_token.clone());
    session.expires_at = Set(new_expires_at);

    session.update(db).await?;

    Ok(new_token)
}
