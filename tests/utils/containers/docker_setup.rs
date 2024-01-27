pub fn generate_database_url(port: u16) -> String {
    format!(
        "postgres://test_user:test_password@localhost:{}/postgres",
        port
    )
}
