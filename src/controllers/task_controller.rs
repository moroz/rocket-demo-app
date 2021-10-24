use crate::models::DbConn;
use crate::models::Task;
use crate::schema::tasks::dsl::*;
use diesel::RunQueryDsl;
use rocket::serde::json::Json;

#[get("/")]
pub async fn list_tasks(conn: DbConn) -> Result<Json<Vec<Task>>, String> {
    conn.run(|c| match tasks.load::<Task>(c) {
        Ok(list) => Ok(Json(list)),
        _ => Err(String::from("No can do")),
    })
    .await
}
