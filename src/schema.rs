// @generated automatically by Diesel CLI.

diesel::table! {
    bets (id) {
        id -> Int4,
        contract_id -> Int4,
        amount -> Numeric,
        status -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

diesel::joinable!(bets -> contracts (contract_id));

diesel::allow_tables_to_appear_in_same_query!(
    bets,
    contracts,
);
