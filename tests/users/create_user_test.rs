use crate::setup::{prepare_test_db, revert_test_db};
use actix_web::{test, App};
use code2media::routes::user::configure;
use env_logger;
use serde_json::json;
use std::env;

#[actix_rt::test]
async fn test_user_creation() {
    let _ = env_logger::try_init();
    env::set_var("RUNNING_TEST", "1");

    prepare_test_db().expect("Test DB setup failed");

    let mut app = test::init_service(App::new().configure(configure)).await;

    let user_data = json!({
        "name": "Test User",
        "email": "test@example.com"
    });

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success(), "Failed to create user");

    revert_test_db().expect("Test DB revert failed");
}
