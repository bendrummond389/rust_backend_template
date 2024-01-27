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

pub fn get_user(user_id: i32) -> Result<User, diesel::result::Error> {
    let mut conn = establish_connection();

    users::table.find(user_id).first(&mut conn)
}
