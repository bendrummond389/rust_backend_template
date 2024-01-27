use crate::db::connection::establish_connection;
use crate::models::user::{User, NewUser};
use diesel::prelude::*;

pub fn create_user(new_user: NewUser) -> Result<User, diesel::result::Error> {
    use crate::schema::users;

    let mut conn = establish_connection(); 
    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn); 

    result
}
