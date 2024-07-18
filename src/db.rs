use crate::models::{Bet, Contract, NewBet, NewContract};
use crate::schema::bets::dsl::*;
use crate::schema::contracts::dsl::*;
use diesel::prelude::*;

pub fn create_contract(conn: &PgConnection, new_contract: NewContract) -> Contract {
    diesel::insert_into(contracts)
        .values(&new_contract)
        .get_result(conn)
        .expect("Error saving new contract")
}

pub fn get_contract(conn: &PgConnection, contract_id: i32) -> Option<Contract> {
    contracts
        .find(contract_id)
        .first::<Contract>(conn)
        .optional()
        .expect("Error loading contract")
}

pub fn get_contracts(conn: &PgConnection) -> Vec<Contract> {
    contracts
        .load::<Contract>(conn)
        .expect("Error loading contracts")
}

pub fn update_contract(
    conn: &PgConnection,
    contract_id: i32,
    new_name: &str,
    new_address: &str,
    new_status: i32,
) -> Option<Contract> {
    diesel::update(contracts.find(contract_id))
        .set((
            name.eq(new_name),
            address.eq(new_address),
            status.eq(new_status),
        ))
        .get_result::<Contract>(conn)
        .optional()
        .expect("Error updating contract")
}

pub fn delete_contract(conn: &PgConnection, contract_id: i32) {
    diesel::delete(contracts.find(contract_id))
        .execute(conn)
        .expect("Error deleting contract");
}

pub fn create_bet(conn: &PgConnection, new_bet: NewBet) -> Bet {
    diesel::insert_into(bets)
        .values(&new_bet)
        .get_result(conn)
        .expect("Error saving new bet")
}

pub fn get_bet(conn: &PgConnection, bet_id: i32) -> Option<Bet> {
    bets.find(bet_id)
        .first::<Bet>(conn)
        .optional()
        .expect("Error loading bet")
}

pub fn get_bets(conn: &PgConnection) -> Vec<Bet> {
    bets.load::<Bet>(conn).expect("Error loading bets")
}

pub fn update_bet(
    conn: &PgConnection,
    bet_id: i32,
    new_contract_id: i32,
    new_amount: f64,
    new_status: i32,
) -> Option<Bet> {
    diesel::update(bets.find(bet_id))
        .set((
            contract_id.eq(new_contract_id),
            amount.eq(new_amount),
            status.eq(new_status),
        ))
        .get_result::<Bet>(conn)
        .optional()
        .expect("Error updating bet")
}

pub fn delete_bet(conn: &PgConnection, bet_id: i32) {
    diesel::delete(bets.find(bet_id))
        .execute(conn)
        .expect("Error deleting bet");
}
