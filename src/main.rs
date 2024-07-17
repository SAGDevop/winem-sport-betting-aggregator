#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod models;
mod schema;

use db::{create_contract, delete_contract, get_contract, get_contracts, update_contract};
use diesel::PgConnection;
use dotenv::dotenv;
use models::{Contract, NewContract};
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_sync_db_pools::database;

#[database("postgres_db")]
struct DbConn(PgConnection);

#[derive(Deserialize)]
struct ContractData {
    name: String,
    address: String,
    status: i32,
}

#[post("/", data = "<contract_data>")]
async fn create(conn: DbConn, contract_data: Json<ContractData>) -> Json<Contract> {
    conn.run(|c| {
        create_contract(
            c,
            NewContract {
                name: &contract_data.name,
                address: &contract_data.address,
                status: contract_data.status,
            },
        )
    })
    .await
    .into()
}

#[get("/<id>")]
async fn read(conn: DbConn, id: i32) -> Option<Json<Contract>> {
    conn.run(move |c| get_contract(c, id)).await.map(Json)
}

#[get("/")]
async fn read_all(conn: DbConn) -> Json<Vec<Contract>> {
    conn.run(|c| get_contracts(c)).await.into()
}

#[put("/<id>", data = "<contract_data>")]
async fn update(
    conn: DbConn,
    id: i32,
    contract_data: Json<ContractData>,
) -> Option<Json<Contract>> {
    conn.run(move |c| {
        update_contract(
            c,
            id,
            &contract_data.name,
            &contract_data.address,
            contract_data.status,
        )
    })
    .await
    .map(Json)
}

#[delete("/<id>")]
async fn delete(conn: DbConn, id: i32) -> status::NoContent {
    conn.run(move |c| delete_contract(c, id)).await;
    status::NoContent
}

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // Cargar variables de entorno desde .env
    rocket::build().attach(DbConn::fairing()).mount(
        "/contracts",
        routes![create, read, read_all, update, delete],
    )
}
