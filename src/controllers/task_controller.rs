use crate::models::Task;
use rocket::serde::json::Json;

#[get("/")]
pub fn list_tasks() -> Json<Vec<Task>> {
    let tasks = vec![
        Task::new(1, "Water the flowers", None),
        Task::new(2, "Feed the dog", Some("Description")),
    ];

    Json(tasks)
}
