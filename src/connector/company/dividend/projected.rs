
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDate;

use super::super::super::super::schema::projected_dividend;
use super::super::super::super::model::ProjectedDividend;

pub fn create (conn: &PgConnection, company: i32, value_date: NaiveDate, index: f64) {
  let new_dividend = ProjectedDividend {
    value_date: value_date,
    company: company,
    index: index,
  };

  diesel::insert_into(projected_dividend::table)
    .values(&new_dividend)
    .execute(conn)
    .expect("Error saving new Dividend");
}

pub fn fetch(conn: &PgConnection, id: i32, day: NaiveDate) -> ProjectedDividend {
  use super::super::super::super::schema::projected_dividend::dsl::*;
  return projected_dividend
    .find((id, day))
    .first::<ProjectedDividend>(conn)
    .expect("Unable to find value");
}