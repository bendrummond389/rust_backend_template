use crate::schema::users;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel::Queryable;
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub name: Option<&'a str>,
    pub email: Option<&'a str>,
}
