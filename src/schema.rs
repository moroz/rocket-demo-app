table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;

    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        done -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;

    users (id) {
        id -> Int4,
        email -> Citext,
        password_hash -> Varchar,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
