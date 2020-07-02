use chrono::NaiveDate;
use diesel::data_types::Cents;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use super::schema;
use super::model::{Entry, NewEntry, Company, NewCompany, NewIndustry, Industry, Dividend, IndustryMap};

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

pub fn create_company (conn: &PgConnection, name: String, shares: i32, stdev: f64, initial_dividend: NaiveDate) -> Company {
  use schema::company;

  let new_company = NewCompany {
    name: name,
    dividend: initial_dividend,
    shares: shares,
    stdev: stdev, 
  };

  return diesel::insert_into(company::table)
    .values(&new_company)
    .get_result(conn)
    .expect("Error saving new company");
}

pub fn create_dividend(conn: &PgConnection, company: i32, date: NaiveDate, payment: f64) {
  use schema::dividend;
  let new_dividend = Dividend {
    announcement_date: NaiveDate::from_num_days_from_ce(0),
    company: company,
    exdividend_date: NaiveDate::from_num_days_from_ce(0),
    payment_date: date,
    payment: payment
  };

  diesel::insert_into(dividend::table)
    .values(&new_dividend)
    .execute(conn)
    .expect("Error saving new Dividend");
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

pub fn create_industry_map (conn: &PgConnection, industry: i32, company: i32, beta: f64, weight: f64) {
  use schema::industry_map;
  let new_industry_map = IndustryMap {
    industry: industry,
    company: company,
    beta: beta,
    weight: weight,
  };
  
  diesel::insert_into(industry_map::table)
    .values(&new_industry_map)
    .execute(conn)
    .expect("Error saving new industry mapping");
}

pub fn clear_industry (conn: &PgConnection) {
  use schema::industry;
  diesel::delete(industry::table)
    .execute(conn)
    .expect("Error clearing industry");
}

pub fn fetch_industries(conn: &PgConnection) -> std::vec::Vec<Industry> {
  use schema::industry::dsl::*;
  return industry.filter(name.ne("Total Market"))
    .load::<Industry>(conn)
    .expect("Error fetching industries");
}
