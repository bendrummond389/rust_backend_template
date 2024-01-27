use crate::schema::users;
use diesel::Insertable;
use diesel::Queryable;
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    // other fields
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    // Include other fields that your users table contains
}
