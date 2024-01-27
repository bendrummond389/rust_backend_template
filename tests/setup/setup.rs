use diesel::connection::Connection;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use log::{error, info};
use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type DB = diesel::pg::Pg;

fn establish_connection(database_url: &str) -> Result<PgConnection, String> {
    PgConnection::establish(&database_url)
        .map_err(|e| format!("Error connecting to database: {}", e))
}

fn run_database_migrations(conn: &mut impl MigrationHarness<DB>) {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => info!("Migrations run successfully"),
        Err(e) => error!("Error reverting migrations: {}", e),
    }
}

fn revert_database_migrations(conn: &mut impl MigrationHarness<DB>) {
    match conn.revert_all_migrations(MIGRATIONS) {
        Ok(_) => info!("Migrations reverted successfully"),
        Err(e) => error!("Error reverting migrations: {}", e),
    }
}

fn get_database_url() -> String {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    database_url
}

pub fn prepare_test_db() -> Result<(), String> {
    let database_url = get_database_url();
    let mut connection = establish_connection(&database_url)?;
    run_database_migrations(&mut connection);
    Ok(())
}

pub fn revert_test_db() -> Result<(), String> {
    let database_url = get_database_url();
    let mut connection = establish_connection(&database_url)?;
    revert_database_migrations(&mut connection);
    Ok(())
}
