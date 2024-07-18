use crate::schema::bets;
use crate::schema::contracts;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Contract {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "contracts"]
pub struct NewContract<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub status: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Bet {
    pub id: i32,
    pub contract_id: i32,
    pub amount: f64,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "bets"]
pub struct NewBet<'a> {
    pub contract_id: i32,
    pub amount: f64,
    pub status: i32,
}
