// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        unionid -> Varchar,
        openid -> Varchar,
        name -> Varchar,
        derive -> Varchar,
        blank -> Bool,
    }
}
