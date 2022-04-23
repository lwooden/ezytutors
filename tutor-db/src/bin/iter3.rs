use actix_web::{web, App, HttpServer, middleware::Logger};
use std::io;
use std::sync::Mutex;
use std::env;
use sqlx::postgres::PgPool;
use dotenv::dotenv;

#[macro_use]
extern crate log;

#[path = "../iter2/models.rs"]
mod models;

#[path = "../iter2/handlers.rs"]
mod handlers;

#[path = "../iter2/routes.rs"]
mod routes;

#[path = "../iter2/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    // source values from .env file and inject them into the environment
    dotenv().ok(); 
    
    // check for presence of DATABASE_URL; if not present throw msg passed to expect
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    
    // create new conneciton pool; this is an async op
    let db_pool = PgPool::new(&database_url).await.unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize AppState object
    let shared_data = web::Data::new(AppState { 
        health_check_response: "I'm good. Stop asking!".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool
    });

    let app = move || {
        App::new()
        .wrap(Logger::default())
        .app_data(shared_data.clone()) // dependency injection for the web application
        .configure(general_routes)
        .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await

}