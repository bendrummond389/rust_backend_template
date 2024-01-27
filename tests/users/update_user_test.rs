use crate::utils::containers::docker_setup::generate_database_url;
use crate::utils::containers::pg_image::PostgresImage;
use crate::utils::establish_connection_and_run_migrations;
use crate::utils::seed_database;
use actix_web::{test, App};
use code2media::routes::user::configure;
use serde_json::json;
use serde_json::Value;
use std::env;

use testcontainers::clients;

#[actix_rt::test]
async fn test_user_update() {
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

    let user_data = json!({
        "name": "Updated User",
        "email": "updated@example.com"
    });

    // Update the user
    let req = test::TestRequest::put()
        .uri(&format!("/users/{}", user_id))
        .set_json(&user_data)
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success(), "Failed to update user");

    // Verify the updated user
    let updated_user: Value = test::read_body_json(resp).await;
    assert_eq!(
        updated_user["id"].as_i64(),
        Some(user_id),
        "Updated user ID does not match"
    );
    assert_eq!(
        updated_user["name"].as_str(),
        Some("Updated User"),
        "Updated user name does not match"
    );
    assert_eq!(
        updated_user["email"].as_str(),
        Some("updated@example.com"),
        "Updated user email does not match"
    );
}
