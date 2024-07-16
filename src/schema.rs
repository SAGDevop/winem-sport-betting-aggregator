// @generated automatically by Diesel CLI.

diesel::table! {
    contracts (id) {
        id -> Int4,
        name -> Varchar,
        order_id -> Varchar,
        status -> Int4,
        address -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
