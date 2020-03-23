#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod model;
pub mod server;


use diesel::data_types::Cents;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::model::{Entry, NewEntry, Company, NewCompany};

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  return PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url));
}


pub fn create_entry (conn: &PgConnection, company: i32, is_buy: bool, price: Cents, quantity: i32, buyer: i32) -> Entry {
  use schema::ledger;

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

pub fn create_company (conn: &PgConnection, name: String) -> Company {
  use schema::company;

  let new_company = NewCompany {
    name: name,
  };

  return diesel::insert_into(company::table)
    .values(&new_company)
    .get_result(conn)
    .expect("Error saving new company");
}
