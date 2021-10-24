use crate::models::DbConn;
use crate::models::User;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NewSessionParams {
    email: String,
    password: String,
}

#[post("/", data = "<params>")]
pub async fn sign_in(conn: DbConn, params: Json<NewSessionParams>) -> Result<Json<User>, String> {
    conn.run(move |c| {
        match User::authenticate_by_email_password(&params.email[..], &params.password[..], c) {
            Some(user) => Ok(Json(user)),
            None => Err(String::from("NO can do")),
        }
    })
    .await
}
