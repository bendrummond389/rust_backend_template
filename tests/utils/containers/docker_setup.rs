use crate::utils::containers::pg_image::PostgresImage;

pub fn prepare_postgres_image() -> PostgresImage {
    PostgresImage::default()
}

pub fn generate_database_url(port: u16) -> String {
    format!(
        "postgres://test_user:test_password@localhost:{}/postgres",
        port
    )
}