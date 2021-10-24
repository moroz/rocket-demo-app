use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}
