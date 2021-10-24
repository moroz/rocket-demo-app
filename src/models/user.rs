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
    pub fn check_pass(&self, password: &str) -> BcryptResult<bool> {
        bcrypt::verify(password, &self.password_hash)
    }

    pub fn authenticate_by_email_password(
        user_email: &str,
        password: &str,
        conn: &PgConnection,
    ) -> Option<Self> {
        let user = users
            .filter(email.eq(CiString::from(user_email)))
            .first::<User>(conn)
            .optional();
        match user {
            Ok(Some(user)) => match user.check_pass(password) {
                Ok(true) => Some(user),
                _ => None,
            },
            _ => None,
        }
    }
}
