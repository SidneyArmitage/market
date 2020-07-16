
use diesel::pg::PgConnection;
use diesel::prelude::*;

use super::super::model::{Industry, NewIndustry};
use super::super::schema::industry;

pub mod map;
pub mod value;

pub fn create (conn: &PgConnection, name: String, beta: f64, stdev: f64) -> Industry {

  let new_industry = NewIndustry {
    name: name,
    beta: beta,
    stdev: stdev,
  };

  return diesel::insert_into(industry::table)
    .values(&new_industry)
    .get_result(conn)
    .expect("Failed to create Industry");
}


pub fn clear (conn: &PgConnection) {
  diesel::delete(industry::table)
    .execute(conn)
    .expect("Error clearing industry");
}

pub fn fetch_all (conn: &PgConnection) -> std::vec::Vec<Industry> {
  use super::super::schema::industry::dsl::*;
  return industry.filter(name.ne("Total Market"))
    .load::<Industry>(conn)
    .expect("Error fetching industries");
}

pub fn fetch_total (conn: &PgConnection) -> Industry {
  use super::super::schema::industry::dsl::*;
  return industry.filter(name.eq("Total Market"))
    .first::<Industry>(conn)
    .expect("Unable to find market");
}