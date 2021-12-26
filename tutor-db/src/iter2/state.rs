use std::sync::Mutex;
use sqlx::postgres::PgPool;

pub struct AppState {
    pub health_check_response: String, // immutable
    pub visit_count: Mutex<u32>, // mutable
    pub db: PgPool, // add a property to persist database conn pool to AppState so it can injected and shared

}