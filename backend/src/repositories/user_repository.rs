use crate::domain::user::NewUser;
use crate::dto::user_dto::UpdateUserDto;
use crate::errors::Error;
use chrono::{Duration, Utc};
use entity::{email_verification_tokens, password_reset_tokens, roles, user_roles, users};
use migration::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, Set, TransactionTrait};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{DeleteResult, QueryFilter};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn create(
    db: &DatabaseConnection,
    new_user: NewUser,
) -> Result<(users::Model, Uuid), Error> {
    db.transaction::<_, (users::Model, Uuid), Error>(|txn| {
        Box::pin(async move {
            let model = users::ActiveModel {
                email: Set(new_user.email),
                ..Default::default()
            };
            let user_model = model.insert(txn).await?;

            let verification_token = Uuid::new_v4();
            email_verification_tokens::ActiveModel {
                user_id: Set(user_model.id),
                token: Set(verification_token),
                ..Default::default()
            }
            .insert(txn)
            .await?;

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
        active_user.preferences = Set(serde_json::to_value(prefs).unwrap());
    }

    Ok(active_user.update(db).await?)
}

pub async fn confirm_email(db: &DatabaseConnection, token: Uuid) -> Result<(), Error> {
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
    token: Uuid,
) -> Result<(), DbErr> {
    // Set expiration for 1 hour from now
    let expires_at = Utc::now() + Duration::hours(1);

    let new_token = password_reset_tokens::ActiveModel {
        user_id: Set(user_id),
        token: Set(token),
        expires_at: Set(expires_at.naive_utc()),
        ..Default::default()
    };

    password_reset_tokens::Entity::insert(new_token)
        .exec(db)
        .await?;
    Ok(())
}
pub async fn find_reset_token_by_token(
    db: &DatabaseConnection,
    token: Uuid,
) -> Result<Option<entity::password_reset_tokens::Model>, Error> {
    let token = password_reset_tokens::Entity::find()
        .filter(password_reset_tokens::Column::Token.eq(token))
        .one(db)
        .await?;
    Ok(token)
}
pub async fn delete_reset_token_by_id(
    db: &DatabaseConnection,
    token_id: Uuid,
) -> Result<DeleteResult, DbErr> {
    password_reset_tokens::Entity::delete_by_id(token_id)
        .exec(db)
        .await
}
pub async fn update_2fa_secret_if_null(
    db: &DatabaseConnection,
    user_id: Uuid,
    secret: String,
) -> Result<(), Error> {
    users::Entity::update_many()
        .col_expr(users::Column::TwoFactorSecret, Expr::value(secret))
        .filter(users::Column::Id.eq(user_id))
        .filter(users::Column::TwoFactorSecret.is_null())
        .exec(db)
        .await?;
    Ok(())
}

pub async fn find_user_by_2fa_token(
    db: &DatabaseConnection,
    token: Uuid,
) -> Result<users::Model, Error> {
    let now = Utc::now().naive_utc();

    users::Entity::find()
        .filter(users::Column::TwoFactorToken.eq(token))
        .filter(users::Column::TwoFactorTokenExpiresAt.gt(now))
        .one(db)
        .await?
        .ok_or(Error::Unauthorized(serde_json::json!({
            "error": "Invalid or expired 2FA token"
        })))
}

pub async fn clear_2fa_token(db: &DatabaseConnection, user_id: Uuid) -> Result<(), Error> {
    users::Entity::update(users::ActiveModel {
        id: Set(user_id),
        two_factor_token: Set(None),
        two_factor_token_expires_at: Set(None),
        ..Default::default()
    })
    .exec(db)
    .await?;
    Ok(())
}

pub async fn update_recovery_codes(
    db: &DatabaseConnection,
    user_id: Uuid,
    codes: Value,
) -> Result<users::Model, Error> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({"error": "User not found"})))?;

    let mut active_user: users::ActiveModel = user.into();

    active_user.two_factor_recovery_codes = Set(Some(codes));

    Ok(active_user.update(db).await?)
}

pub async fn set_2fa_status(
    db: &DatabaseConnection,
    user_id: Uuid,
    enabled: bool,
) -> Result<(), Error> {
    let confirmed_at = if enabled {
        Some(Utc::now().naive_utc())
    } else {
        None
    };

    users::Entity::update(users::ActiveModel {
        id: Set(user_id),
        two_factor_confirmed_at: Set(confirmed_at),
        ..Default::default()
    })
    .exec(db)
    .await?;
    Ok(())
}

pub async fn disable_2fa(db: &DatabaseConnection, user_id: Uuid) -> Result<(), Error> {
    users::Entity::update(users::ActiveModel {
        id: Set(user_id),
        two_factor_secret: Set(None),
        two_factor_recovery_codes: Set(None),
        two_factor_confirmed_at: Set(None),
        ..Default::default()
    })
    .exec(db)
    .await?;
    Ok(())
}
pub async fn consume_recovery_code(
    db: &DatabaseConnection,
    user_id: Uuid,
    provided_code: &str,
) -> Result<bool, Error> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(json!({"error": "User not found"})))?;

    // 2. Parse the JSON recovery codes
    let mut codes: Vec<String> = match user.clone().two_factor_recovery_codes {
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => return Ok(false),
    };

    if let Some(pos) = codes.iter().position(|c| c == provided_code) {
        codes.remove(pos);

        let mut active_user: users::ActiveModel = user.into();
        active_user.two_factor_recovery_codes = Set(Some(json!(codes)));
        active_user.update(db).await?;

        return Ok(true);
    }

    Ok(false)
}
