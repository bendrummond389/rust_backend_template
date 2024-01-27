use crate::utils::containers::docker_setup::generate_database_url;
use crate::utils::containers::pg_image::PostgresImage;
use crate::utils::establish_connection_and_run_migrations;
use crate::utils::seed_database;
use actix_web::{test, App};
use code2media::routes::user::configure;
use serde_json::Value;
use std::env;

use testcontainers::clients;

#[actix_rt::test]
async fn test_user_fetching() {
    let docker = clients::Cli::default();
    let image = PostgresImage::default();
    let container = docker.run(image);
    let port = container.get_host_port_ipv4(5432);
    let user_id = 1;

    let database_url = generate_database_url(port);
    env::set_var("DATABASE_URL", &database_url);

    establish_connection_and_run_migrations(&database_url);

    seed_database(&database_url);

    let mut app = test::init_service(App::new().configure(configure)).await;

    // Fetch the created user
    let fetch_req = test::TestRequest::get()
        .uri(&format!("/users/{}", user_id))
        .to_request();
    let fetch_resp = test::call_service(&mut app, fetch_req).await;
    assert!(fetch_resp.status().is_success(), "Failed to fetch user");

    // Verify the fetched user
    let fetched_user: Value = test::read_body_json(fetch_resp).await;
    assert_eq!(
        fetched_user["id"].as_i64(),
        Some(user_id),
        "Fetched user ID does not match"
    );
}
