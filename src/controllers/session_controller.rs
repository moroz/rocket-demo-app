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
    let password = String::from(&params.password);

    let res = conn
        .run(move |c| User::get_by_email(&params.email[..], c))
        .await;

    match res {
        Some(user) => match user.check_pass(password).await {
            Ok(true) => Ok(Json(user)),
            _ => Err(String::from("NO can do")),
        },
        _ => Err(String::from("NO can do")),
    }
}
