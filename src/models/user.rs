use crate::models::token::SessionPayload;
use crate::schema::users::dsl::*;
use bcrypt::BcryptResult;
use diesel::prelude::*;
use diesel_citext::types::CiString;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub inserted_at: SystemTime,
    pub updated_at: SystemTime,
}

impl User {
    pub fn get_by_email(user_email: &str, conn: &PgConnection) -> Option<Self> {
        users
            .filter(email.eq(CiString::from(user_email)))
            .first::<User>(conn)
            .optional()
            .unwrap()
    }

    pub async fn check_pass(&self, password: String) -> BcryptResult<bool> {
        let hash = String::from(&self.password_hash);
        tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash[..]))
            .await
            .unwrap()
    }
}

impl From<&User> for SessionPayload {
    fn from(user: &User) -> Self {
        Self { sub: user.id }
    }
}
