use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{error, info};

type DB = diesel::pg::Pg;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection_and_run_migrations(database_url: &str) -> PgConnection {
    let mut connection = PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    run_database_migrations(&mut connection);
    connection
}

fn run_database_migrations(conn: &mut impl MigrationHarness<DB>) {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => info!("Migrations run successfully"),
        Err(e) => error!("Error running migrations: {}", e),
    }
}
