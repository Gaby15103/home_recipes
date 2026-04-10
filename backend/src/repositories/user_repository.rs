use crate::domain::user::NewUser;
use crate::dto::user_dto::ProfileDto;
use crate::errors::Error;
use chrono::{Duration, Utc};
use entity::{email_verification_tokens, password_reset_tokens, roles, user_roles, users};
use migration::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, Set, TransactionError, TransactionTrait, UpdateResult};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{DeleteResult, QueryFilter};
use serde_json::{Value, json};
use uuid::Uuid;
use entity::users::Model;

pub async fn create(
    db: &DatabaseConnection,
    new_user: NewUser,
) -> Result<(Model, Uuid), TransactionError<Error>> {
    db.transaction::<_, (users::Model, Uuid), Error>(|txn| {
        Box::pin(async move {
            let model = users::ActiveModel {
                email: Set(new_user.email.clone()),
                username: Set(new_user.username.clone()),
                first_name: Set(new_user.first_name.clone()),
                last_name: Set(new_user.last_name.clone()),
                password_hash: Set(new_user.password_hash.clone()),
                avatar_url: Set(new_user.avatar_url.clone()),
                preferences: Set(new_user.preferences.clone()),
                ..Default::default()
            };

            let user_model = model
                .insert(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                    "message": "Failed to insert new user",
                    "operation": "create",
                    "entity": "users",
                    "email": &new_user.email,
                    "username": &new_user.username,
                    "error": e.to_string(),
                    "stage": "user_insert"
                })))?;

            let verification_token = Uuid::new_v4();
            email_verification_tokens::ActiveModel {
                user_id: Set(user_model.id),
                token: Set(verification_token),
                ..Default::default()
            }
                .insert(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to create email verification token",
                "operation": "create",
                "entity": "email_verification_tokens",
                "user_id": user_model.id.to_string(),
                "error": e.to_string(),
                "stage": "token_creation"
            })))?;

            let user_role = roles::Entity::find()
                .filter(roles::Column::Name.eq("USER"))
                .one(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                    "message": "Failed to fetch USER role",
                    "operation": "create",
                    "entity": "roles",
                    "role_name": "USER",
                    "error": e.to_string(),
                    "stage": "role_lookup"
                })))?
                .ok_or_else(|| Error::InternalServerError(json!({
                    "message": "USER role not found in database",
                    "operation": "create",
                    "entity": "roles",
                    "role_name": "USER",
                    "stage": "role_validation"
                })))?;

            user_roles::ActiveModel {
                user_id: Set(user_model.id),
                role_id: Set(user_role.id),
                ..Default::default()
            }
                .insert(txn)
                .await
                .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to assign USER role to new user",
                "operation": "create",
                "entity": "user_roles",
                "user_id": user_model.id.to_string(),
                "role_id": user_role.id.to_string(),
                "error": e.to_string(),
                "stage": "role_assignment"
            })))?;

            Ok((user_model, verification_token))
        })
    })
        .await
        .map_err(|e| {
            log::error!("Transaction failed in user creation: {:?}", e);
            e
        })
}

pub async fn email_exists(db: &DatabaseConnection, email: &str) -> Result<bool, Error> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to check if email exists",
            "operation": "email_exists",
            "entity": "users",
            "email": email,
            "error": e.to_string(),
            "stage": "query"
        })))?;

    Ok(user.is_some())
}

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<users::Model, Error> {
    users::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch user by ID",
            "operation": "find_by_id",
            "entity": "users",
            "user_id": id.to_string(),
            "error": e.to_string(),
            "stage": "query"
        })))?
        .ok_or_else(|| Error::InternalServerError(json!({
            "message": "User not found",
            "operation": "find_by_id",
            "entity": "users",
            "user_id": id.to_string(),
            "stage": "validation"
        })))
}

pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Result<users::Model, Error> {
    users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch user by email",
            "operation": "find_by_email",
            "entity": "users",
            "email": email,
            "error": e.to_string(),
            "stage": "query"
        })))?
        .ok_or_else(|| Error::InternalServerError(json!({
            "message": "User not found",
            "operation": "find_by_email",
            "entity": "users",
            "email": email,
            "stage": "validation"
        })))
}

pub async fn reset_password(
    db: &DatabaseConnection,
    user_id: Uuid,
    new_hash: String,
) -> Result<(), Error> {
    let mut user: users::ActiveModel = users::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch user for password reset",
            "operation": "reset_password",
            "entity": "users",
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?
        .ok_or_else(|| Error::InternalServerError(json!({
            "message": "User not found for password reset",
            "operation": "reset_password",
            "entity": "users",
            "user_id": user_id.to_string(),
            "stage": "validation"
        })))?
        .into();

    user.password_hash = Set(new_hash);

    user.update(db).await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to update user password",
            "operation": "reset_password",
            "entity": "users",
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "update"
        })))?;

    Ok(())
}

pub async fn update_user_profile(
    db: &DatabaseConnection,
    user_id: Uuid,
    data: ProfileDto,
) -> Result<Model, Error> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({ "error": e.to_string() })))?
        .ok_or_else(|| Error::NotFound(json!({ "message": "User not found" })))?;

    let mut active_user: users::ActiveModel = user.into();

    active_user.username = Set(data.username);
    active_user.first_name = Set(data.first_name);
    active_user.last_name = Set(data.last_name);
    active_user.avatar_url = Set(data.avatar_url);

    active_user.preferences = Set(serde_json::to_value(data.preferences)
        .map_err(|e| Error::InternalServerError(json!({ "error": e.to_string() })))?);

    let updated_user = active_user.update(db).await.map_err(|e| {
        Error::InternalServerError(json!({ "message": "Save failed", "error": e.to_string() }))
    })?;

    Ok(updated_user)
}

pub async fn confirm_email(db: &DatabaseConnection, token: Uuid) -> Result<(), Error> {
    let token_record = email_verification_tokens::Entity::find()
        .filter(email_verification_tokens::Column::Token.eq(token))
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch email verification token",
            "operation": "confirm_email",
            "entity": "email_verification_tokens",
            "token": token.to_string(),
            "error": e.to_string(),
            "stage": "token_fetch"
        })))?;

    if let Some(record) = token_record {
        users::Entity::update_many()
            .col_expr(users::Column::EmailVerified, Expr::value(true))
            .filter(users::Column::Id.eq(record.user_id))
            .exec(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to mark email as verified",
                "operation": "confirm_email",
                "entity": "users",
                "user_id": record.user_id.to_string(),
                "error": e.to_string(),
                "stage": "user_update"
            })))?;

        email_verification_tokens::Entity::delete_by_id(record.id)
            .exec(db)
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to delete verification token after confirmation",
                "operation": "confirm_email",
                "entity": "email_verification_tokens",
                "token_id": record.id.to_string(),
                "user_id": record.user_id.to_string(),
                "error": e.to_string(),
                "stage": "token_delete"
            })))?;
    }

    Ok(())
}

pub async fn create_reset_token(
    db: &DatabaseConnection,
    user_id: Uuid,
    token: Uuid,
) -> Result<(), DbErr> {
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
    let token_record = password_reset_tokens::Entity::find()
        .filter(password_reset_tokens::Column::Token.eq(token))
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch password reset token",
            "operation": "find_reset_token_by_token",
            "entity": "password_reset_tokens",
            "token": token.to_string(),
            "error": e.to_string(),
            "stage": "query"
        })))?;

    Ok(token_record)
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
) -> Result<UpdateResult, Error> {
    users::Entity::update_many()
        .col_expr(users::Column::TwoFactorSecret, Expr::value(secret.clone()))
        .filter(users::Column::Id.eq(user_id))
        .filter(users::Column::TwoFactorSecret.is_null())
        .exec(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to update 2FA secret",
            "operation": "update_2fa_secret_if_null",
            "entity": "users",
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "update"
        })))
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
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch user by 2FA token",
            "operation": "find_user_by_2fa_token",
            "entity": "users",
            "token": token.to_string(),
            "error": e.to_string(),
            "stage": "query"
        })))?
        .ok_or_else(|| Error::Unauthorized(json!({
            "error": "Invalid or expired 2FA token",
            "operation": "find_user_by_2fa_token",
            "token": token.to_string(),
            "stage": "validation"
        })))
}

pub async fn clear_2fa_token(db: &DatabaseConnection, user_id: Uuid) -> Result<Model, Error> {
    users::Entity::update(users::ActiveModel {
        id: Set(user_id),
        two_factor_token: Set(None),
        two_factor_token_expires_at: Set(None),
        ..Default::default()
    })
        .exec(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to clear 2FA token",
        "operation": "clear_2fa_token",
        "entity": "users",
        "user_id": user_id.to_string(),
        "error": e.to_string(),
        "stage": "update"
    })))
}

pub async fn update_recovery_codes(
    db: &DatabaseConnection,
    user_id: Uuid,
    codes: Value,
) -> Result<users::Model, Error> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch user for recovery codes update",
            "operation": "update_recovery_codes",
            "entity": "users",
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?
        .ok_or_else(|| Error::InternalServerError(json!({
            "message": "User not found for recovery codes update",
            "operation": "update_recovery_codes",
            "entity": "users",
            "user_id": user_id.to_string(),
            "stage": "validation"
        })))?;

    let mut active_user: users::ActiveModel = user.into();
    active_user.two_factor_recovery_codes = Set(Some(codes));

    active_user.update(db).await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to update recovery codes",
            "operation": "update_recovery_codes",
            "entity": "users",
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "update"
        })))
}

pub async fn set_2fa_status(
    db: &DatabaseConnection,
    user_id: Uuid,
    enabled: bool,
) -> Result<Model, Error> {
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
        .await
        .map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to update 2FA status",
        "operation": "set_2fa_status",
        "entity": "users",
        "user_id": user_id.to_string(),
        "enabled": enabled,
        "error": e.to_string(),
        "stage": "update"
    })))
}

pub async fn disable_2fa(db: &DatabaseConnection, user_id: Uuid) -> Result<Model, Error> {
    users::Entity::update(users::ActiveModel {
        id: Set(user_id),
        two_factor_secret: Set(None),
        two_factor_recovery_codes: Set(None),
        two_factor_confirmed_at: Set(None),
        ..Default::default()
    })
        .exec(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to disable 2FA",
        "operation": "disable_2fa",
        "entity": "users",
        "user_id": user_id.to_string(),
        "error": e.to_string(),
        "stage": "update"
    })))
}

pub async fn consume_recovery_code(
    db: &DatabaseConnection,
    user_id: Uuid,
    provided_code: &str,
) -> Result<bool, Error> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to fetch user for recovery code consumption",
            "operation": "consume_recovery_code",
            "entity": "users",
            "user_id": user_id.to_string(),
            "error": e.to_string(),
            "stage": "fetch"
        })))?
        .ok_or_else(|| Error::InternalServerError(json!({
            "message": "User not found for recovery code consumption",
            "operation": "consume_recovery_code",
            "entity": "users",
            "user_id": user_id.to_string(),
            "stage": "validation"
        })))?;

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

        active_user.update(db).await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to update recovery codes after consumption",
                "operation": "consume_recovery_code",
                "entity": "users",
                "user_id": user_id.to_string(),
                "error": e.to_string(),
                "stage": "update"
            })))?;

        return Ok(true);
    }

    Ok(false)
}