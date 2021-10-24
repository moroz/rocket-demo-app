use crate::controllers;
use rocket::Build;
use rocket::Rocket;

pub fn build() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/tasks",
            routes![
                controllers::task_controller::list_tasks,
                controllers::task_controller::get_task
            ],
        )
        .mount(
            "/sessions",
            routes![controllers::session_controller::sign_in],
        )
}
