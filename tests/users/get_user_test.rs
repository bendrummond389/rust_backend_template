use actix_web::{test, App};
use code2media::routes::user::configure;
use serde_json::json;
use serde_json::Value;
use std::env;

#[actix_rt::test]
async fn test_user_fetching() {
    let _ = env_logger::try_init();
    env::set_var("RUNNING_TEST", "1");

    let mut app = test::init_service(App::new().configure(configure)).await;

    let user_data = json!({
      "name": "Test User",
      "email": "test@example.com"
    });

    let create_req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();
    let create_resp = test::call_service(&mut app, create_req).await;
    assert!(create_resp.status().is_success(), "Failed to create user");

    let created_user: Value = test::read_body_json(create_resp).await;
    let user_id = created_user["id"].as_i64().expect("Failed to get user ID");

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
