
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDate;

use super::super::schema;
use super::super::model::Dividend;

pub fn create (conn: &PgConnection, company: i32, date: NaiveDate, announcement: NaiveDate, exdividend: NaiveDate, payment: f64) {
  use schema::dividend;
  let new_dividend = Dividend {
    announcement_date: announcement,
    company: company,
    exdividend_date: exdividend,
    payment_date: date,
    payment: payment
  };

  diesel::insert_into(dividend::table)
    .values(&new_dividend)
    .execute(conn)
    .expect("Error saving new Dividend");
}
