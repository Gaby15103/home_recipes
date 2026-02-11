use crate::domain::user::NewUser;
use crate::dto::user_dto::UpdateUserDto;
use crate::errors::Error;
use chrono::Utc;
use entity::prelude::{EmailVerificationTokens, Users};
use entity::users::Model;
use entity::{email_verification_tokens, roles, user_roles, users};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, Set, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;

pub async fn create(
    db: &DatabaseConnection,
    new_user: NewUser,
) -> Result<(users::Model, Uuid), Error> {
    db.transaction::<_, (users::Model, Uuid), Error>(|txn| {
        Box::pin(async move {
            // 1. Insert User
            let model = users::ActiveModel {
                email: Set(new_user.email),
                // ... rest of fields
                ..Default::default()
            };
            let user_model = model.insert(txn).await?;

            // 2. Insert Verification Token
            let verification_token = Uuid::new_v4();
            email_verification_tokens::ActiveModel {
                user_id: Set(user_model.id),
                token: Set(verification_token),
                ..Default::default()
            }
            .insert(txn)
            .await?;

            // 3. Assign Role
            let user_role = roles::Entity::find()
                .filter(roles::Column::Name.eq("USER"))
                .one(txn)
                .await?
                .ok_or(Error::InternalServerError)?;

            user_roles::ActiveModel {
                user_id: Set(user_model.id),
                role_id: Set(user_role.id),
            }
            .insert(txn)
            .await?;

            Ok((user_model, verification_token))
        })
    })
    .await
    .map_err(|e| Error::from(e))
}

pub async fn email_exists(db: &DatabaseConnection, email: &str) -> Result<bool, Error> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await?;
    Ok(user.is_some())
}

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<users::Model, Error> {
    users::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({
            "error":"User Not Found"
        })))
}

pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Result<users::Model, Error> {
    users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({
            "error":"User Not Found"
        })))
}

pub async fn reset_password(
    db: &DatabaseConnection,
    user_id: Uuid,
    new_hash: String,
) -> Result<(), Error> {
    let mut user: users::ActiveModel = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({
            "error":"User Not Found"
        })))?
        .into();
    user.password_hash = Set(new_hash);

    user.update(db).await?;

    Ok(())
}

pub async fn update_user_profile(
    db: &DatabaseConnection,
    user_id: Uuid,
    data: UpdateUserDto,
) -> Result<users::Model, Error> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({"error": "User not found"})))?;

    let mut active_user: users::ActiveModel = user.into();

    if let Some(first_name) = data.first_name {
        active_user.first_name = Set(first_name);
    }
    if let Some(last_name) = data.last_name {
        active_user.last_name = Set(last_name);
    }
    if let Some(avatar) = data.avatar_url {
        active_user.avatar_url = Set(avatar);
    }
    if let Some(prefs) = data.preferences {
        // Ensure you use the proper conversion for your JSON field
        active_user.preferences = Set(serde_json::to_value(prefs).unwrap());
    }

    Ok(active_user.update(db).await?)
}
