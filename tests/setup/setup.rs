use diesel::connection::Connection;
use diesel::migration::{MigrationSource, MigrationVersion};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type DB = diesel::pg::Pg;

pub fn run_db_migrations(conn: &mut impl MigrationHarness<DB>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

pub fn revert_db_migrations(conn: &mut impl MigrationHarness<DB>) {
    conn.revert_all_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

pub fn prepare_test_db() -> Result<(), String> {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    let mut connection = PgConnection::establish(&database_url).expect("failed to connect to db");

    run_db_migrations(&mut connection);
    Ok(())
}

pub fn revert_test_db() -> Result<(), String> {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    let mut connection = PgConnection::establish(&database_url).expect("failed to connect to db");
    revert_db_migrations(&mut connection);
    Ok(())
}
