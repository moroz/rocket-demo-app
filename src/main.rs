#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate diesel_citext;
extern crate hmac;
extern crate jwt;
extern crate sha2;
extern crate tokio;

mod controllers;
mod models;
mod router;
mod schema;

use crate::models::DbConn;

#[launch]
fn rocket() -> _ {
    router::build().attach(DbConn::fairing())
}
