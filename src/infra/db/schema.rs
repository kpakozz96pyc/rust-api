// @generated automatically by Diesel CLI.

diesel::table! {
    kills (id) {
        id -> Uuid,
        killer -> Varchar,
        killed -> Varchar,
    }
}
