use actix::prelude::*;
use diesel::prelude::*;
use libreauth::pass::HashBuilder;

use super::DbExecutor;
use crate::app::users::{LoginUser, RegisterUser, UpdateUserOuter, UserResponse};
use crate::db::roles::fetch_roles_for_user;
use crate::models::{NewUser, User, UserChange};
use crate::prelude::*;
use crate::utils::{HASHER, PWD_SCHEME_VERSION};

impl Message for RegisterUser {
    type Result = Result<UserResponse>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let mut conn = self.0.get()?; // PooledConnection

        let new_user = NewUser {
            email: msg.email,
            username: msg.username,
            password_hash: HASHER.hash(&msg.password)?,
            first_name: msg.first_name,
            last_name: msg.last_name,
            avatar_url: Some("/assets/users/default.png".parse().unwrap()),
            preferences: serde_json::json!({}),
        };

        let inserted_user: User = diesel::insert_into(users)
            .values(&new_user)
            .get_result(&mut conn)?;

        let roles = Vec::new();

        Ok(UserResponse::from_user_and_roles(inserted_user, roles))
    }
}

impl Message for LoginUser {
    type Result = Result<UserResponse>;
}

impl Handler<LoginUser> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: LoginUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let provided_password_raw = &msg.password;

        let mut conn = self.0.get()?;

        let stored_user: User = users.filter(email.eq(msg.email)).first(&mut conn)?;
        println!("{:?}", stored_user);
        let checker = HashBuilder::from_phc(stored_user.password_hash.trim())?;

        if checker.is_valid(provided_password_raw) {
            if checker.needs_update(Option::from(PWD_SCHEME_VERSION)) {
                let new_password = HASHER.hash(provided_password_raw)?;
                let updated_user: User = diesel::update(users.find(stored_user.id))
                    .set(password_hash.eq(new_password))
                    .get_result(&mut conn)?;

                let user_roles = fetch_roles_for_user(&mut conn, updated_user.id)?;

                return Ok(UserResponse::from_user_and_roles(updated_user, user_roles));
            }

            let user_roles = fetch_roles_for_user(&mut conn, stored_user.id)?;
            Ok(UserResponse::from_user_and_roles(stored_user, user_roles))
        } else {
            Err(Error::Unauthorized(json!({"error": "Wrong password"})))
        }
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

        let updated_user_data  = UserChange {
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

        Ok(UserResponse::from_user_and_roles(updated_user, user_roles))
    }
}
