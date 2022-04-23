

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vector<Course> {

 // Build SQL statement
  course_rows = sqlquery!(
      "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 where tutor_id = $1", 
      tutor_id
    )
    .fetch_all(pool) // pass in the database pool and execute the query
    .await // mark as an async op
    .unwrap(); // unwrap the result

    // Extract results and convert the query results into a Rust Vector 
    course_rows
      .iter()
      .map(|course_row| Course { // this is similar to array.map functionality in Node
          course_id: course_row.course_id,
          tutor_id: course_row.tutor_id,
          course_name: course_row.course_name.clone(),
          posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time))
      })
      .collect() // packages up the final Vector to be returned to the function caller  
}