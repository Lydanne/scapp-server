// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        openid -> Varchar,
        name -> Varchar,
        derive -> Varchar,
        blank -> Bool,
    }
}
