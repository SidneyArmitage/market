
use diesel::pg::PgConnection;
use diesel::query_dsl::QueryDsl;
use diesel::prelude::*;
use chrono::NaiveDate;
use diesel::dsl::count_star;

use super::super::model::{Company, NewCompany};
use super::super::schema::company;

pub mod dividend;

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

pub struct CompanyIteratable<'a> {
  connector: &'a PgConnection,
  length: i64,
  point: i64,
  query: company::table,
}

impl Iterator for CompanyIteratable<'_>
{
  type Item = Company;

  fn next(&mut self) -> Option<Company> {
    self.point += 1;
    if self.point == self.length {
      return None;
    }
    return Some(self.query
      .offset(self.point)
      .first::<Company>(self.connector)
      .expect("Error loading company"));
  }
}

pub fn fetch_all(conn: &PgConnection) -> CompanyIteratable {
  let count = company::dsl::company.select(count_star())
    .first(conn)
    .expect("Error loading count");
  return CompanyIteratable {
    connector: conn,
    point: 0,
    length: count,
    query: company::dsl::company,
  }
}

pub fn clear(conn: &PgConnection) {
  dividend::clear(conn);
  diesel::delete(company::dsl::company).execute(conn).expect("Unable to clear companies");
}