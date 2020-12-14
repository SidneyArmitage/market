
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDate;

use super::super::super::schema::dividend;
use super::super::super::model::Dividend;

pub mod projected;

pub fn create (conn: &PgConnection, company: i32, date: NaiveDate, announcement: NaiveDate, exdividend: NaiveDate, payment: f64) {
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

pub fn fetch(conn: &PgConnection, id: i32, day: NaiveDate) -> Dividend {
  use super::super::super::schema::dividend::dsl::*;
  return dividend
    .find((id, day))
    .first::<Dividend>(conn)
    .expect("Unable to find value");
}

pub fn update(conn: &PgConnection, id: i32, day: NaiveDate, value: f64) {
  use super::super::super::schema::dividend::dsl::*;
  diesel::update(dividend.find((id, day)))
    .set(payment.eq(value))
    .execute(conn)
    .expect("failed to update");
}

pub fn clear(conn: &PgConnection) {
  use super::super::super::schema::dividend::dsl::*;
  projected::clear(conn);
  diesel::delete(dividend).execute(conn).expect("Unable to clear dividends");
}