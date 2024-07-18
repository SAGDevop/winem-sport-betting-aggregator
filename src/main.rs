#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod models;
mod schema;

use db::{
    create_bet, create_contract, delete_bet, delete_contract, get_bet, get_bets, get_contract,
    get_contracts, update_bet, update_contract,
};
use diesel::PgConnection;
use dotenv::dotenv;
use models::{Bet, Contract, NewBet, NewContract};
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

#[derive(Deserialize)]
struct BetData {
    contract_id: i32,
    amount: f64,
    status: i32,
}

#[post("/contracts", data = "<contract_data>")]
async fn create_contract_handler(
    conn: DbConn,
    contract_data: Json<ContractData>,
) -> Json<Contract> {
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

#[get("/contracts/<id>")]
async fn read_contract(conn: DbConn, id: i32) -> Option<Json<Contract>> {
    conn.run(move |c| get_contract(c, id)).await.map(Json)
}

#[get("/contracts")]
async fn read_all_contracts(conn: DbConn) -> Json<Vec<Contract>> {
    conn.run(|c| get_contracts(c)).await.into()
}

#[put("/contracts/<id>", data = "<contract_data>")]
async fn update_contract_handler(
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

#[delete("/contracts/<id>")]
async fn delete_contract_handler(conn: DbConn, id: i32) -> status::NoContent {
    conn.run(move |c| delete_contract(c, id)).await;
    status::NoContent
}

#[post("/bets", data = "<bet_data>")]
async fn create_bet_handler(conn: DbConn, bet_data: Json<BetData>) -> Json<Bet> {
    conn.run(|c| {
        create_bet(
            c,
            NewBet {
                contract_id: bet_data.contract_id,
                amount: bet_data.amount,
                status: bet_data.status,
            },
        )
    })
    .await
    .into()
}

#[get("/bets/<id>")]
async fn read_bet(conn: DbConn, id: i32) -> Option<Json<Bet>> {
    conn.run(move |c| get_bet(c, id)).await.map(Json)
}

#[get("/bets")]
async fn read_all_bets(conn: DbConn) -> Json<Vec<Bet>> {
    conn.run(|c| get_bets(c)).await.into()
}

#[put("/bets/<id>", data = "<bet_data>")]
async fn update_bet_handler(conn: DbConn, id: i32, bet_data: Json<BetData>) -> Option<Json<Bet>> {
    conn.run(move |c| {
        update_bet(
            c,
            id,
            bet_data.contract_id,
            bet_data.amount,
            bet_data.status,
        )
    })
    .await
    .map(Json)
}

#[delete("/bets/<id>")]
async fn delete_bet_handler(conn: DbConn, id: i32) -> status::NoContent {
    conn.run(move |c| delete_bet(c, id)).await;
    status::NoContent
}

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // Cargar variables de entorno desde .env
    rocket::build().attach(DbConn::fairing()).mount(
        "/",
        routes![
            create_contract_handler,
            read_contract,
            read_all_contracts,
            update_contract_handler,
            delete_contract_handler,
            create_bet_handler,
            read_bet,
            read_all_bets,
            update_bet_handler,
            delete_bet_handler
        ],
    )
}
