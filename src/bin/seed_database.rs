extern crate code2media;

use code2media::utils::seed::seed_database;

fn main() {
    let database_url = "postgres://test_user:test_password@localhost:5433/postgres_test";
    seed_database(database_url);
}
