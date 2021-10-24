#[database("pg")]
pub struct DbConn(diesel::PgConnection);
