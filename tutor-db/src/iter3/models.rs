use actix_web::web; 
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize}; 
 
// The annotation #derive derives the implementations for specific traits
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: usize,
    pub course_id: Option<usize>, // Option type means the field can hold a valid value of type usize or NONE
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>
}

// From trait allows me to parse incoming json body from a web request and map to a Course object
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time
        }
    }
}

