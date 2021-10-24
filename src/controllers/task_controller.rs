use crate::models::DbConn;
use crate::models::Task;
use rocket::serde::json::Json;

#[get("/")]
pub async fn list_tasks(conn: DbConn) -> Result<Json<Vec<Task>>, String> {
    conn.run(|c| match Task::all(c) {
        Ok(list) => Ok(Json(list)),
        _ => Err(String::from("No can do")),
    })
    .await
}

#[get("/<task_id>")]
pub async fn get_task(task_id: i32, conn: DbConn) -> Option<Json<Task>> {
    conn.run(move |c| match Task::get(task_id, &c) {
        Some(task) => Some(Json(task)),
        None => None,
    })
    .await
}
