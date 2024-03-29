use crate::utils::containers::docker_setup::generate_database_url;
use crate::utils::containers::pg_image::PostgresImage;
use crate::utils::establish_connection_and_run_migrations;
use crate::utils::seed_database;
use actix_web::{test, App};
use code2media::routes::user::configure;
use serde_json::json;
use std::env;
use testcontainers::clients;

#[actix_rt::test]
async fn test_user_create() {
    let docker = clients::Cli::default();
    let image = PostgresImage::default();
    let container = docker.run(image);
    let port = container.get_host_port_ipv4(5432);

    let database_url = generate_database_url(port);
    env::set_var("DATABASE_URL", &database_url);

    establish_connection_and_run_migrations(&database_url);

    seed_database(&database_url);

    let mut app = test::init_service(App::new().configure(configure)).await;

    let user_data = json!({
        "name": "Test User",
        "email": "test@example.com"
    });

    // Create a request for user creation
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();

    // Call service with the request
    let resp = test::call_service(&mut app, req).await;

    // Assert the response
    assert!(resp.status().is_success(), "Failed to create user");
}
