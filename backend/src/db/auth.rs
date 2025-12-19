use actix::prelude::*;
use diesel::prelude::*;

use crate::db::DbExecutor;
use crate::db::roles::fetch_roles_for_user;
use crate::models::{Role, User};
use crate::prelude::*;
use crate::utils::{
    auth::{Auth, GenerateAuth},
    jwt::CanDecodeJwt,
};

// message handler implementations ↓

impl Message for GenerateAuth {
    type Result = Result<Auth>;
}

impl Handler<GenerateAuth> for DbExecutor {
    type Result = Result<Auth>;

    fn handle(&mut self, msg: GenerateAuth, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let claims = msg.token.decode_jwt()?.claims;

        let mut conn = self.0.get()?;

        let user: User = users.find(claims.id).first(&mut conn)?;

        let roles: Vec<Role> = fetch_roles_for_user(&mut conn, user.id)?;

        Ok(Auth {
            user,
            token: msg.token,
            roles,
        })
    }
}
