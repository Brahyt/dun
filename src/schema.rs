// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        message -> Varchar,
    }
}
