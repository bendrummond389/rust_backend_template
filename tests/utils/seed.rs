use code2media::schema::users;
use code2media::user::NewUser;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn seed_database(database_url: &str) {
    let mut connection =
        PgConnection::establish(database_url).expect("failed to connect to database");

    let users = vec![
        NewUser {
            name: "John Doe",
            email: "john@example.com",
        },
        NewUser {
            name: "Jane Doe",
            email: "jane@example.com",
        },
    ];

    for user in users {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(&mut connection)
            .expect("Error seeding user");
    }
}
