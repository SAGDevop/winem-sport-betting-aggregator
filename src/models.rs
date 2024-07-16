use super::schema::contracts;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::Queryable;

#[derive(Queryable, Insertable)]
#[table_name = "contracts"]
pub struct Contract {
    pub id: i32,
    pub name: String,
    pub order_id: String,
    pub address: String,
    pub status: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
