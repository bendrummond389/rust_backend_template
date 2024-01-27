
pub mod controllers;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

// If you want to re-export items for direct use in main.rs
pub use db::*;
pub use models::*;
pub use crate::routes::user::configure;