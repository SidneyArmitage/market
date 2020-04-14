#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod model;
pub mod util;


use diesel::data_types::Cents;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDate;
use self::model::{Entry, NewEntry, Company, NewCompany, NewIndustry, Industry};

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

pub fn create_company (conn: &PgConnection, name: String, shares: i32, alpha: f64, stdev: f64) -> Company {
  use schema::company;

  let new_company = NewCompany {
    name: name,
    dividend: NaiveDate::from_num_days_from_ce(0),
    shares: shares,
    alpha: alpha,
    stdev: stdev, 
  };

  return diesel::insert_into(company::table)
    .values(&new_company)
    .get_result(conn)
    .expect("Error saving new company");
}

pub fn create_industry (conn: &PgConnection, name: String, beta: f64, stdev: f64) {
  use schema::industry;

  let new_industry = NewIndustry {
    name: name,
    beta: beta,
    stdev: stdev,
  };

  diesel::insert_into(industry::table)
    .values(&new_industry)
    .execute(conn)
    .expect("Error saving new industry");
}

pub fn clear_industry (conn: &PgConnection) {
  use schema::industry;
  diesel::delete(industry::table)
    .execute(conn)
    .expect("Error clearing industry");
}

pub fn fetch_industries(conn: &PgConnection) -> std::vec::Vec<model::Industry> {
  use schema::industry::dsl::*;
  return industry.filter(name.ne("Total Market"))
    .load::<Industry>(conn)
    .expect("Error fetching industries");
}