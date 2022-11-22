// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
