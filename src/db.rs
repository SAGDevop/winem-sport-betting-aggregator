use crate::models::{Contract, NewContract};
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
