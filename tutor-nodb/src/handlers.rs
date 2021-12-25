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
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .collect::<Vec<Course>>()
        .len();

    println!("{}",course_count_for_user.to_string());

    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

// Get All Courses For A Tutor Handler
pub async fn get_all_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<usize>) -> HttpResponse {

    let tutor_id: usize = params.0;
    println!("Fetching courses for tutor {}!", tutor_id);

    let filtered_courses = app_state
        .courses
        .lock() // courses property is protected by Mutex; we must issue a lock in order to modify it
        .unwrap() // subsequently we must unwrap
        .clone()
        .into_iter() // convert courses into an iterator 
        .filter(|course| course.tutor_id == tutor_id) // a closure (or anonymous function) with 1 parameter
        .collect::<Vec<Course>>(); // turn filtered result into a collection

        if filtered_courses.len() > 0 {
            HttpResponse::Ok().json(filtered_courses)
        } else {
            HttpResponse::Ok().json("No courses found for tutor".to_string())
        }
}

pub async fn get_course_detail(app_state: web::Data<AppState>, params: web::Path<(usize,usize)>) -> HttpResponse {

    let (tutor_id, course_id) = params.0;

    let selected_courses = app_state
      .courses
      .lock()
      .unwrap()
      .clone()
      .into_iter()
      .find(|x| x.tutor_id == tutor_id && x.course_id == Some(course_id)) // Some is used here because course_id is of type Option
      .ok_or("Course not found!");

    // The `if let` construct reads: "if `let` destructures `selected_course` into
    // `Ok(course)`, evaluate the block (`{}`).
      if let Ok(course) = selected_courses {
          HttpResponse::Ok().json(course)
      } else {
          HttpResponse::Ok().json("Course not found".to_string())
      }
}


// Test Suite

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course  = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: None,
            posted_time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let resp = new_course(app_state, course).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_for_tutor_test() {

        // Initialize AppState
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        // Simulate parameters based on the Course object we created above
        let tutor_id: web::Path<usize> = web::Path::from(1);

        // Call handler and supply the newly updated app_state along with params for the request
        let resp = get_all_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let params: web::Path<(usize,usize)> = web::Path::from((1,1));

        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}