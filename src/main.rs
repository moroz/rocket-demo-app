#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod router;

#[launch]
fn rocket() -> _ {
    router::build()
}
