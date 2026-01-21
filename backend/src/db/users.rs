use super::DbExecutor;
use crate::app::users::DeleteSession;
use crate::db::roles::fetch_roles_for_user;
use crate::dto::{ConfirmEmail, EmailVerificationTokenResponse, LoginUser, RegisterResponse, RegisterUser, UpdateUserOuter, UserResponse, UserResponseOuter};
use crate::models::{
    EmailVerificationToken, NewEmailVerificationToken, NewSession, NewUser, Session, User,
    UserChange,
};
use crate::prelude::*;
use crate::utils::{HASHER, PWD_SCHEME_VERSION};
use actix::prelude::*;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use libreauth::pass::HashBuilder;
use uuid::Uuid;
use crate::utils::email_service::send_email_confirmation;

impl Message for RegisterUser {
    type Result = Result<RegisterResponse>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<RegisterResponse>;

    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::email_verification_tokens::dsl::*;
        use crate::schema::users::dsl::*;
        use diesel::Connection;

        let mut conn = self.0.get()?; // PooledConnection

        conn.transaction::<RegisterResponse, Error, _>(|conn| {
            // 1️⃣ Create new user
            let new_user = NewUser {
                email: msg.email,
                username: msg.username,
                password_hash: HASHER.hash(&msg.password)
                    .map_err(|e| Error::InternalServerError)?,
                first_name: msg.first_name,
                last_name: msg.last_name,
                avatar_url: Some("/assets/users/default.png".parse().unwrap()),
                preferences: serde_json::json!({}),
            };

            let inserted_user: User = diesel::insert_into(users)
                .values(&new_user)
                .get_result(conn)
                .map_err(Error::from)?;

            // 2️⃣ Create email verification token
            let verification_token = Uuid::new_v4();
            let new_token = NewEmailVerificationToken {
                user_id: inserted_user.id,
                token: verification_token,
                created_at: Utc::now(),
            };

            let inserted_token: EmailVerificationToken = diesel::insert_into(email_verification_tokens)
                .values(&new_token)
                .get_result(conn)
                .map_err(Error::from)?;

            let roles = Vec::new();
            let response = RegisterResponse {
                user: UserResponse::from_user_and_roles(&inserted_user, roles),
                email_verification_tokens: EmailVerificationTokenResponse::from_email_verification_token(inserted_token),
            };

            // 3️⃣ Send email and propagate error to rollback
            send_email_confirmation(
                response.user.user.clone(),
                &response.email_verification_tokens.token.to_string()
            )?;

            Ok(response)
        })
    }
}



impl Message for ConfirmEmail {
    type Result = Result<(), Error>;
}

impl Handler<ConfirmEmail> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: ConfirmEmail, _: &mut Self::Context) -> Self::Result {
        use crate::schema::email_verification_tokens::dsl::{
            email_verification_tokens,
            id as token_id_col,
            token as token_col,
        };
        use crate::schema::users::dsl::{users, id as user_id_col, email_verified};

        let mut conn = self.0.get()?;

        let token_uuid = uuid::Uuid::parse_str(&msg.token).map_err(|_| Error::InternalServerError)?;

        // Get the token record
        let record: EmailVerificationToken = email_verification_tokens
            .filter(token_col.eq(token_uuid))
            .first(&mut conn)?;

        // Mark the user as verified
        diesel::update(users.filter(user_id_col.eq(record.user_id)))
            .set(email_verified.eq(true))
            .execute(&mut conn)?;

        // Delete the token
        diesel::delete(email_verification_tokens.filter(token_id_col.eq(record.id)))
            .execute(&mut conn)?;

        Ok(())
    }
}


impl Message for LoginUser {
    type Result = Result<UserResponseOuter, Error>;
}

impl Handler<LoginUser> for DbExecutor {
    type Result = Result<UserResponseOuter, Error>;

    fn handle(&mut self, msg: LoginUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::sessions::dsl::sessions;
        use crate::schema::users::dsl::*;

        let mut conn = self.0.get()?; // pooled connection

        let stored_user: User = users
            .filter(email.eq(&msg.email))
            .first(&mut conn)?;

        let checker = HashBuilder::from_phc(stored_user.password_hash.trim())?;
        if !checker.is_valid(&msg.password) {
            return Err(Error::Forbidden(json!({
                "error": "Wrong password."
            })));
        }

        if !stored_user.email_verified.unwrap_or(false) {
            return Err(Error::Forbidden(json!({
                "error": "Email not verified."
            })));
        }

        if checker.needs_update(Some(PWD_SCHEME_VERSION)) {
            let new_hash = HASHER.hash(&msg.password)?;
            diesel::update(users.find(stored_user.id))
                .set(password_hash.eq(new_hash))
                .execute(&mut conn)?;
        }

        let two_factor_required = stored_user.two_factor_secret.is_some()
            && stored_user.two_factor_confirmed_at.is_some();

        if two_factor_required {
            let token = Uuid::new_v4();
            let expires = Utc::now() + Duration::minutes(10);

            diesel::update(users.find(stored_user.id))
                .set((
                    two_factor_token.eq(token),
                    two_factor_token_expires_at.eq(expires),
                ))
                .execute(&mut conn)?;

            return Ok(UserResponseOuter {
                two_factor_required: true,
                two_factor_token: Some(token),
                user: None,
                session_id: Uuid::nil(),
            });
        }

        let roles = fetch_roles_for_user(&mut conn, stored_user.id)?;

        let expires_at = Utc::now() + Duration::days(30);
        let new_session = NewSession {
            user_id: stored_user.id,
            expires_at,
        };
        let session: Session = diesel::insert_into(sessions)
            .values(&new_session)
            .get_result(&mut conn)?;

        Ok(UserResponseOuter {
            user: Some(UserResponse::from_user_and_roles(&stored_user, roles)),
            session_id: session.id,
            two_factor_required,
            two_factor_token: None,
        })
    }
}


impl Message for DeleteSession {
    type Result = Result<()>;
}

impl Handler<DeleteSession> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: DeleteSession, _: &mut Self::Context) -> Self::Result {
        use crate::schema::sessions::dsl::*;

        let mut conn = self.0.get()?;

        diesel::delete(sessions.filter(id.eq(msg.session_id))).execute(&mut conn)?;

        Ok(())
    }
}

impl Message for UpdateUserOuter {
    type Result = Result<UserResponse>;
}

impl Handler<UpdateUserOuter> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: UpdateUserOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let auth = msg.auth;
        let update_user = msg.update_user;

        let mut conn = self.0.get()?;

        let updated_password = HASHER.hash(&update_user.password)?;

        let updated_user_data = UserChange {
            username: update_user.username,
            email: update_user.email,
            password_hash: updated_password,
            first_name: update_user.first_name,
            last_name: update_user.last_name,
            avatar_url: update_user.avatar_url,
            preferences: Default::default(),
        };

        let updated_user = diesel::update(users.find(auth.user.id))
            .set(&updated_user_data)
            .get_result::<User>(&mut conn)?;

        let user_roles = fetch_roles_for_user(&mut conn, updated_user.id)?;

        Ok(UserResponse::from_user_and_roles(&updated_user, user_roles))
    }
}
