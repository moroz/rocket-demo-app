table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        done -> Bool,
    }
}
