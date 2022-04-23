use super::state::AppState;
use super::models::Course;
use actix_web::{web, HttpResponse};
use chrono::Utc;

// Health Check Handler
pub async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {

    info!("Got a request!");

    // read application state 
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count); 

    // update vist_count property
    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

// New Course Handler
pub async fn new_course(app_state: web::Data<AppState>, new_course: web::Json<Course>) -> HttpResponse {

    println!("Got a new course!");
    HttpResponse::Ok().json("success")
}

// Get All Courses For A Tutor Handler
pub async fn get_all_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<usize>) -> HttpResponse {

    let tutor_id: usize = params.0;
    println!("Fetching courses for tutor {}!", tutor_id);
    HttpResponse::Ok().json("success")
}

pub async fn get_course_detail(app_state: web::Data<AppState>, params: web::Path<(usize,usize)>) -> HttpResponse {

    let (tutor_id, course_id) = params.0;

    HttpResponse::Ok().json("success")
}


// Test Suite
#[cfg(test)]
mod tests {
   use super::*;
   use actix_web::http::StatusCode;
   use chrono::NaiveDate;
   use dotenv::dotenv;
   use sqlx::postgres::PgPool;
   use std::env;
   use std::sync::Mutex;

   #[actix_rt::test]
   async fn get_all_courses_success() {
       dotenv().ok();
       let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
       let pool: PgPool = PgPool::new(&database_url).await.unwrap();
       let app_state: web::Data<AppState> = web::Data::new(AppState {
           health_check_response: "".to_string(),
           visit_count: Mutex::new(0),
           db: pool,
       });
       let tutor_id: web::Path<usize> = web::Path::from(1);
       let resp = get_all_courses_for_tutor(app_state, tutor_id).await;
       assert_eq!(resp.status(), StatusCode::OK);
   }

   #[actix_rt::test]
   async fn get_course_detail_test() {
       dotenv().ok();
       let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
       let pool: PgPool = PgPool::new(&database_url).await.unwrap();
       let app_state: web::Data<AppState> = web::Data::new(AppState {
           health_check_response: "".to_string(),
           visit_count: Mutex::new(0),
           db: pool,
       });
       let params: web::Path<(usize, usize)> = web::Path::from((1, 2));
       let resp = get_course_detail(app_state, params).await;
       assert_eq!(resp.status(), StatusCode::OK);
   }

   #[actix_rt::test]
   async fn post_course_success() {
       dotenv().ok();
       let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
       let pool: PgPool = PgPool::new(&database_url).await.unwrap();
       let app_state: web::Data<AppState> = web::Data::new(AppState {
           health_check_response: "".to_string(),
           visit_count: Mutex::new(0),
           db: pool,
       });
       let new_course_msg = Course {
           course_id: 1,
           tutor_id: 1,
           course_name: "This is the next course".into(),
           posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(14, 01, 11)),
       };
       let course_param = web::Json(new_course_msg);
       let resp = new_course(app_state, course_param).await;
       assert_eq!(resp.status(), StatusCode::OK);
   }
}