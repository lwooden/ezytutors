use actix_web::{web, App, HttpServer, middleware::Logger};
use std::io;
use std::sync::Mutex;

#[macro_use]
extern crate log;

#[path = "../models.rs"]
mod models;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routes.rs"]
mod routes;

#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize state object and set visit count to 0
    let shared_data = web::Data::new(AppState { 
        health_check_response: "I'm good. Stop asking!".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let app = move || {
        App::new()
        .wrap(Logger::default())
        .app_data(shared_data.clone())
        .configure(general_routes)
        .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await

}