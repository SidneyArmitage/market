use diesel::data_types::Cents;
use chrono::NaiveDate;
use super::schema::ledger;
use super::schema::company;
use super::schema::industry;
use super::schema::industry_map;
use super::schema::dividend;

#[derive(Queryable)]
pub struct Entry {
  pub id: i32,
  pub company: i32,
  pub is_buy: bool,
  pub price: Cents,
  pub quantity: i32,
  pub buyer: i32,
}

#[derive(Insertable)]
#[table_name="ledger"]
pub struct NewEntry {
  pub company: i32,
  pub is_buy: bool,
  pub price: Cents,
  pub quantity: i32,
  pub buyer: i32,
}

#[derive(Queryable)]
pub struct Company {
  pub id: i32,
  pub name: String,
  pub dividend: NaiveDate,
  pub shares: i32,
  pub stdev: f64,
}

#[derive(Insertable)]
#[table_name="company"]
pub struct NewCompany {
  pub name: String,
  pub dividend: NaiveDate,
  pub shares: i32,
  pub stdev: f64,
}

#[derive(Queryable)]
pub struct Industry {
  pub id: i32,
  pub name: String,
  pub beta: f64,
  pub stdev: f64,
}

#[derive(Insertable)]
#[table_name="industry"]
pub struct NewIndustry {
  pub name: String,
  pub beta: f64,
  pub stdev: f64,
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="dividend"]
pub struct Dividend {
  pub company: i32,
  pub payment_date: NaiveDate,
  pub announcement_date: NaiveDate,
  pub exdividend_date: NaiveDate,
  pub payment: f64,
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="industry_map"]
pub struct Industry_Map {
  pub industry: i32,
  pub company: i32,
  pub beta: f64,
  pub weight: f64,
}