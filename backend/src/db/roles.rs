use crate::diesel::ExpressionMethods;
use diesel::{JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;
use crate::models::Role;
use crate::schema::{roles, user_roles};

pub fn fetch_roles_for_user(
    conn: &mut PgConnection,
    user_id: Uuid,
) -> Result<Vec<Role>, diesel::result::Error> {
    roles::table
        .inner_join(user_roles::table.on(user_roles::role_id.eq(roles::id)))
        .filter(user_roles::user_id.eq(user_id))
        .select(Role::as_select())
        .load(conn)
        .map_err(Into::into)
}
