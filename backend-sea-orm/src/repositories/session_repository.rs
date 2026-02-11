use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, RelationTrait, QuerySelect, Set, ActiveModelTrait};
use entity::{sessions, users};
use chrono::Utc;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub async fn find_valid_session_with_user(
    db: &DatabaseConnection,
    token: &str,
) -> Result<Option<(sessions::Model, users::Model)>, sea_orm::DbErr> {
    // find_also_related returns Option<(sessions::Model, Option<users::Model>)>
    let result = sessions::Entity::find()
        .find_also_related(users::Entity)
        .filter(sessions::Column::Token.eq(token))
        .filter(sessions::Column::ExpiresAt.gt(Utc::now()))
        .filter(sessions::Column::IsRevoked.eq(false))
        .one(db)
        .await?;

    // Map the nested Option safely
    Ok(result.and_then(|(session, user_opt)| {
        user_opt.map(|user| (session, user))
    }))
}
pub async fn get_user_sessions(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<sessions::Model>, sea_orm::DbErr> {
    sessions::Entity::find()
        .filter(sessions::Column::UserId.eq(user_id))
        .filter(sessions::Column::IsRevoked.eq(false))
        // Only return sessions that haven't expired yet
        .filter(sessions::Column::ExpiresAt.gt(Utc::now()))
        .all(db)
        .await
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
