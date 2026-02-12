use crate::domain::user::NewUser;
use crate::dto::user_dto::UpdateUserDto;
use crate::errors::Error;
use chrono::{Duration, Utc};
use entity::prelude::{EmailVerificationTokens, Users};
use entity::users::Model;
use entity::{email_verification_tokens, password_reset_tokens, roles, user_roles, users};
use sea_orm::{DeleteResult, QueryFilter};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, Set, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;
use migration::Expr;

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
) -> Result<entity::users::Model, Error> {
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

pub async fn confirm_email(
    db: &DatabaseConnection,
    token: Uuid,
)-> Result<(), Error> {
    let token_record = email_verification_tokens::Entity::find()
        .filter(email_verification_tokens::Column::Token.eq(token))
        .one(db)
        .await?;
    if let Some(record) = token_record {
        users::Entity::update_many()
            .col_expr(users::Column::EmailVerified, Expr::value(true))
            .filter(users::Column::Id.eq(record.user_id))
            .exec(db)
            .await?;

        email_verification_tokens::Entity::delete_by_id(record.id)
            .exec(db)
            .await?;
    }

    Ok(())
}
pub async fn create_reset_token(
    db: &DatabaseConnection,
    user_id: Uuid,
    token: Uuid
) -> Result<(), DbErr> {
    // Set expiration for 1 hour from now
    let expires_at = Utc::now() + Duration::hours(1);

    let new_token = password_reset_tokens::ActiveModel {
        user_id: Set(user_id),
        token: Set(token),
        expires_at: Set(expires_at.naive_utc()),
        ..Default::default()
    };

    password_reset_tokens::Entity::insert(new_token).exec(db).await?;
    Ok(())
}
pub async fn find_reset_token_by_token(
    db: &DatabaseConnection,
    token: Uuid
) -> Result<Option<entity::password_reset_tokens::Model>, Error> {
    let token = password_reset_tokens::Entity::find()
        .filter(password_reset_tokens::Column::Token.eq(token))
        .one(db)
    .await?;
    Ok(token)
}
pub async fn delete_reset_token_by_id(
    db: &DatabaseConnection,
    token_id: Uuid
)-> Result<DeleteResult, DbErr> {
    password_reset_tokens::Entity::delete_by_id(token_id).exec(db).await
}