use diesel::pg::PgConnection;
use chrono::NaiveDate;

use super::connector::industry::fetch_total;
use super::connector::industry::value::{fetch, create};
use super::util::random;

pub fn industry(conn: &PgConnection, rng: &mut GenerateGaussian, scale: f64, date: NaiveDate) {
  // get market
  let market_id = fetch_total(conn).id;
  let market_value = fetch(conn, market_id, date).value;
  // set market - next date
  create(conn, market_id, date, walk(rng, scale, market_value));
  // get every industry
  
  // iterate for each
}

pub fn company () {
  // for each company
}

fn walk (rng: &mut GenerateGaussian, scale: f64, value: f64) -> f64 {
  return rng.generate(1f64, scale) * value;
}