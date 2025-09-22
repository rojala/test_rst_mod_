
// Example Rust REST API server logic
// using CodeWhisperer suggestions to enhance productivity

use actix_web::{web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_users() -> impl Responder {
    "List of users"
}