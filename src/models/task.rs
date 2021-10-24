use crate::schema::tasks;
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

impl Task {
    pub fn get(task_id: i32, conn: &PgConnection) -> Option<Self> {
        match tasks::table.find(task_id).first::<Self>(conn) {
            Ok(task) => Some(task),
            _ => None,
        }
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Task>> {
        tasks.load::<Task>(conn)
    }
}
