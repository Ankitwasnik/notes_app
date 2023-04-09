// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Int4,
        text -> Varchar,
        published -> Bool,
    }
}
