use actix_web::{
    App, HttpResponse, HttpServer,
    web::{self, Html},
};
use std::fs;

mod config;
mod handler;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/health",
                web::get().to(async || HttpResponse::Ok().body("SERVER IS RUNNING")),
            )
            .route(
                "/",
                web::get().to(async || {
                    let html = fs::read_to_string("web/index.html").expect("Error loading page");

                    Html::new(html)
                }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
