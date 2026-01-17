use chrono::{DateTime, Utc};
use uuid::Uuid;
use diesel::prelude::*;

use crate::schema::{email_verification_tokens};

use crate::models::{User};

#[derive(Debug, Clone, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = email_verification_tokens)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = email_verification_tokens)]
pub struct NewEmailVerificationToken {
    pub user_id: Uuid,
    pub token: Uuid,
    pub created_at: DateTime<Utc>,
}