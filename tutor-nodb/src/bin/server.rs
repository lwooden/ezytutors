use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
// use env_logger::Env;
use std::io;

#[macro_use]
extern crate log;

// routes
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

// handlers
pub async fn health_check() -> impl Responder {
    info!("Got a request!");
    HttpResponse::Ok().body("Ezytutor service is healthy!")
}

// HTTP Server
#[actix_rt::main]
async fn main() -> io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app = move || App::new()
    .wrap(Logger::default())
    .configure(general_routes);

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}