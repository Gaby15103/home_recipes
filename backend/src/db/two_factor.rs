use actix::prelude::*;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbExecutor;
use crate::db::roles::fetch_roles_for_user;
use crate::dto::{UpdateUserRecoveryCodes, UpdateUserTwoFactorDisable, UpdateUserTwoFactorEnabled, UpdateUserTwoFactorSecret, UserResponse, UserResponseOuter, VerifyTwoFactor, VerifyTwoFactorRequest, VerifyTwoFactorResult};
use crate::error::DbError;
use crate::models::{Session, User};
use crate::prelude::*;

impl Message for UpdateUserTwoFactorSecret {
    type Result = Result<(), DbError>;
}

impl Handler<UpdateUserTwoFactorSecret> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: UpdateUserTwoFactorSecret, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let mut conn = self.0.get()?;
        diesel::update(users.find(msg.user_id))
            .set(two_factor_secret.eq(Some(msg.secret)))
            .execute(&mut conn)?;

        Ok(())
    }
}

impl Message for UpdateUserRecoveryCodes {
    type Result = Result<(),DbError>;
}

impl Handler<UpdateUserRecoveryCodes> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: UpdateUserRecoveryCodes, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let mut conn = self.0.get()?;
        diesel::update(users.find(msg.user_id))
            .set(two_factor_recovery_codes.eq(Some(msg.codes)))
            .execute(&mut conn)?;

        Ok(())
    }
}

impl Message for UpdateUserTwoFactorEnabled {
    type Result = Result<(), DbError>;
}

impl Handler<UpdateUserTwoFactorEnabled> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: UpdateUserTwoFactorEnabled, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let mut conn = self.0.get()?;
        diesel::update(users.find(msg.user_id))
            .set(two_factor_confirmed_at.eq(if msg.enabled { Some(chrono::Utc::now().naive_utc()) } else { None }))
            .execute(&mut conn)?;

        Ok(())
    }
}

impl Message for UpdateUserTwoFactorDisable {
    type Result = Result<(), DbError>;
}

impl Handler<UpdateUserTwoFactorDisable> for DbExecutor {
    type Result = Result<(), DbError>;

    fn handle(&mut self, msg: UpdateUserTwoFactorDisable, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let mut conn = self.0.get()?;
        diesel::update(users.find(msg.user_id))
            .set((
                two_factor_secret.eq::<Option<String>>(None),
                two_factor_recovery_codes.eq::<Option<serde_json::Value>>(None),
                two_factor_confirmed_at.eq::<Option<chrono::NaiveDateTime>>(None),
            ))
            .execute(&mut conn)?;

        Ok(())
    }
}

impl Message for VerifyTwoFactor {
    type Result = Result<VerifyTwoFactorResult>;
}

impl actix::Handler<VerifyTwoFactor> for DbExecutor {
    type Result = Result<VerifyTwoFactorResult>;

    fn handle(&mut self, msg: VerifyTwoFactor, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        use crate::schema::sessions::dsl::sessions;

        let VerifyTwoFactorRequest { token, code, recovery_code } = msg.0;
        let mut conn = self.0.get()?;

        let now = chrono::Utc::now().naive_utc();
        let user: User = users
            .filter(two_factor_token.eq(token))
            .filter(two_factor_token_expires_at.gt(now))
            .first(&mut conn)
            .map_err(|_| Error::Unauthorized(json!({"error": "Invalid or expired 2FA token"})))?;

        let secret = user.two_factor_secret
            .as_ref()
            .ok_or_else(|| Error::Unauthorized(json!({"error": "2FA not enabled"})))?;

        let valid = if let Some(code) = code {
            crate::utils::two_factor::verify_totp(secret, &code)?
        } else if let Some(recovery) = recovery_code {
            crate::utils::two_factor::consume_recovery_code(&mut conn, user.id, &recovery)?
        } else {
            false
        };

        if !valid {
            return Err(Error::Unauthorized(json!({"error": "Invalid authentication code"})));
        }

        diesel::update(users.find(user.id))
            .set((
                two_factor_token.eq::<Option<Uuid>>(None),
                two_factor_token_expires_at.eq::<Option<chrono::NaiveDateTime>>(None),
            ))
            .execute(&mut conn)?;

        let expires_at = chrono::Utc::now() + chrono::Duration::days(30);
        let session: Session = diesel::insert_into(sessions)
            .values((
                crate::schema::sessions::user_id.eq(user.id),
                crate::schema::sessions::expires_at.eq(expires_at),
            ))
            .get_result(&mut conn)?;

        let roles = fetch_roles_for_user(&mut conn, user.id)?;

        Ok(VerifyTwoFactorResult {
            user: UserResponse::from_user_and_roles(&user, roles),
            session_id: session.id,
        })
    }
}
