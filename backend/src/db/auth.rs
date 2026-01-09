use actix::prelude::*;
use chrono::Utc;
use diesel::prelude::*;

use crate::db::DbExecutor;
use crate::db::roles::fetch_roles_for_user;
use crate::models::{Session, User};
use crate::prelude::*;
use crate::schema::users::dsl::users;
use crate::utils::{
    auth::{GetSessionAuth},
};
use crate::utils::auth::SessionAuth;

impl Message for GetSessionAuth {
    type Result = Result<SessionAuth>;
}

impl Handler<GetSessionAuth> for DbExecutor {
    type Result = Result<SessionAuth>;

    fn handle(&mut self, msg: GetSessionAuth, _: &mut Self::Context) -> Self::Result {
        use crate::schema::sessions::dsl::*;
        let mut conn = self.0.get()?; // pooled connection
        
        let session: Session = sessions
            .find(msg.session_id)
            .first(&mut conn)?;
        
        if session.expires_at < Utc::now() {
            return Err(Error::Unauthorized(json!({"error": "Session expired"})));
        }
        
        let user: User = users
            .find(session.user_id)
            .first(&mut conn)?;
        
        let roles = fetch_roles_for_user(&mut conn, user.id)?;

        Ok(SessionAuth {
            session_id: session.id,
            user,
            roles,
        })
    }
}
