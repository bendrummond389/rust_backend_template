use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    // Check if we are in a test environment
    let database_url = match env::var("RUNNING_TEST") {
        Ok(_) => env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set"),
        Err(_) => env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
