use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDate;

use super::super::super::model::IndustryValue;
use super::super::super::schema::industry_value;

pub fn create (conn: &PgConnection, industry: i32, date: NaiveDate, value: f64) {
  let new_industry_value = IndustryValue {
    industry: industry,
    date: date,
    value: value
  };
  
  diesel::insert_into(industry_value::table)
    .values(&new_industry_value)
    .execute(conn)
    .expect("Error saving new industry mapping");
}

pub fn fetch (conn: &PgConnection, sector: i32, day: NaiveDate) -> IndustryValue {
  use super::super::super::schema::industry_value::dsl::*;
  return industry_value
    .find((sector, day))
    .first::<IndustryValue>(conn)
    .expect("Unable to find value");
}

pub fn fetch_from_date (conn: &PgConnection, day: NaiveDate) -> std::vec::Vec<IndustryValue> {
  use super::super::super::schema::industry_value::dsl::*;
  return industry_value
    .filter(date.eq(day))
    .load::<IndustryValue>(conn)
    .expect("unable to fetch date");
}

pub fn clear(conn: &PgConnection) {
  use super::super::super::schema::industry_value::dsl::*;
  diesel::delete(industry_value).execute(conn).expect("Unable to clear value");
}