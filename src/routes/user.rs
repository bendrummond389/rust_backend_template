use crate::controllers::user_controller;
use crate::models::user::NewUser;
use crate::user::UpdateUser;
use actix_web::web::Path;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

// Define a data structure for incoming user data
#[derive(Deserialize)]
pub struct UserData {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct UpdateUserData {
    name: Option<String>,
    email: Option<String>,
}

pub async fn create_user_endpoint(user_data: web::Json<UserData>) -> impl Responder {
    // Convert UserData to NewUser
    let new_user = NewUser {
        name: &user_data.name,
        email: &user_data.email,
    };

    // Call the controller function to create a user
    match user_controller::create_user(new_user) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user_endpoint(user_id: Path<i32>) -> impl Responder {
    match user_controller::get_user(user_id.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_user_endpoint(
    user_id: web::Path<i32>,
    user_data: web::Json<UpdateUserData>,
) -> impl Responder {
    let updated_user = UpdateUser {
        name: user_data.name.as_deref(),
        email: user_data.email.as_deref(),
    };

    match user_controller::update_user(user_id.into_inner(), updated_user) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::post().to(create_user_endpoint)))
        .service(
            web::resource("/users/{id}")
                .route(web::get().to(get_user_endpoint))
                .route(web::put().to(update_user_endpoint)),
        );
}
