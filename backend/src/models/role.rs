use diesel::{AsChangeset, AsExpression, Associations, FromSqlRow, HasQuery, Identifiable, Insertable, QueryId, Queryable, QueryableByName, Selectable, SqlType};
use diesel::expression::ValidGrouping;
use serde_derive::{Deserialize, Serialize};
use crate::schema::{roles};
// -----------------------------
// Role DB Model
// -----------------------------
#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

// Insertable for creating new roles
#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}