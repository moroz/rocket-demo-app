use crate::models::DbConn;
use crate::models::Task;
use crate::schema::tasks;
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/")]
pub async fn list_tasks(conn: DbConn) -> Result<Json<Vec<Task>>, String> {
    conn.run(|c| match tasks.load::<Task>(c) {
        Ok(list) => Ok(Json(list)),
        _ => Err(String::from("No can do")),
    })
    .await
}

#[get("/<task_id>")]
pub async fn get_task(task_id: i32, conn: DbConn) -> Option<Json<Task>> {
    conn.run(
        move |c| match tasks::table.find(&task_id).first::<Task>(c) {
            Ok(task) => Some(Json(task)),
            _ => None,
        },
    )
    .await
}
