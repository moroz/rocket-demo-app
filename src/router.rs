use crate::controllers;
use rocket::Build;
use rocket::Rocket;

pub fn build() -> Rocket<Build> {
    rocket::build().mount("/tasks", routes![controllers::task_controller::list_tasks])
}
