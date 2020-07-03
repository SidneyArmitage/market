use diesel::pg::PgConnection;
use diesel::data_types::Cents;
use diesel::prelude::*;

use super::super::model::{Entry, NewEntry};
use super::super::schema::ledger;

pub fn create (conn: &PgConnection, company: i32, is_buy: bool, price: Cents, quantity: i32, buyer: i32) -> Entry {

  let new_entry = NewEntry {
    company: company,
    is_buy: is_buy,
    price: price,
    quantity: quantity,
    buyer: buyer,
  };

  return diesel::insert_into(ledger::table)
    .values(&new_entry)
    .get_result(conn)
    .expect("Error saving new entry");
}