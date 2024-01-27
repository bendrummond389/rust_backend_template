use actix_web::{web, App, HttpServer};
use code2media::routes::user::configure;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || App::new().service(web::scope("/api").configure(configure)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
