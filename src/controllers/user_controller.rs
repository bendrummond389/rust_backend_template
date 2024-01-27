use crate::db::connection::establish_connection;
use crate::models::user::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;

pub fn create_user(new_user: NewUser) -> Result<User, diesel::result::Error> {
    let mut conn = establish_connection();

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
}
