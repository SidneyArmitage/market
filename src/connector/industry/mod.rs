
use diesel::pg::PgConnection;
use diesel::prelude::*;

use super::super::model::{Industry, NewIndustry};
use super::super::schema::industry;

pub mod map;

pub fn create (conn: &PgConnection, name: String, beta: f64, stdev: f64) {

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


pub fn clear (conn: &PgConnection) {
  diesel::delete(industry::table)
    .execute(conn)
    .expect("Error clearing industry");
}

pub fn fetch (conn: &PgConnection) -> std::vec::Vec<Industry> {
  use super::super::schema::industry::dsl::*;
  return industry.filter(name.ne("Total Market"))
    .load::<Industry>(conn)
    .expect("Error fetching industries");
}
