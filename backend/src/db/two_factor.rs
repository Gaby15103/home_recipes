use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbExecutor;
use crate::dto::{UpdateUserRecoveryCodes, UpdateUserTwoFactorDisable, UpdateUserTwoFactorEnabled, UpdateUserTwoFactorSecret};
use crate::error::DbError;
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
