use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{sessions};

#[derive(Queryable, Insertable, Identifiable, Debug, Selectable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}