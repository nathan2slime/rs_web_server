use crate::database::schemas::user::User;
use diesel::prelude::*;
use crate::database::schemas::schema::users::dsl::*;

pub fn get_users(
    conn: &mut SqliteConnection,
) -> Result<Vec<User>, diesel::result::Error> {
    let data = users.select(User::as_select()).limit(4).load::<User>(conn)?;

    Ok(data)
}
