
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDate;

use super::super::model::{Company, NewCompany};
use super::super::schema::company;

pub fn create (conn: &PgConnection, name: String, shares: i32, stdev: f64, initial_dividend: NaiveDate) -> Company {
  
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