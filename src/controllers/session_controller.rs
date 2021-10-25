use crate::models::DbConn;
use crate::models::SessionPayload;
use crate::models::User;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NewSessionParams {
    email: String,
    password: String,
}

#[post("/", data = "<params>")]
pub async fn sign_in(conn: DbConn, params: Json<NewSessionParams>) -> String {
    let password = String::from(&params.password);

    let res = conn
        .run(move |c| User::get_by_email(&params.email[..], c))
        .await;
    let error = "No can do".to_string();

    match res {
        Some(user) => match user.check_pass(password).await {
            Ok(true) => {
                let payload = SessionPayload::from(&user);
                payload.generate_and_sign().unwrap_or(error)
            }
            _ => error,
        },
        _ => error,
    }
}
