use crate::diesel::ExpressionMethods;
use diesel::{JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;
use crate::error::Error;
use crate::models::Role;
use crate::schema::{roles, user_roles};

pub fn fetch_roles_for_user(
    conn: &mut PgConnection,
    user_id: Uuid,
) -> Result<Vec<Role>, Error> {
    use crate::schema::{roles, user_roles};

    let role_ids = user_roles::table
        .select(user_roles::role_id)
        .filter(user_roles::user_id.eq(user_id))
        .load::<Uuid>(conn)?;

    // Now fetch roles by their IDs
    let roles_for_user = roles::table
        .filter(roles::id.eq_any(role_ids))
        .load::<Role>(conn)?;

    Ok(roles_for_user)
}


