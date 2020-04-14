use diesel::data_types::Cents;
use chrono::NaiveDate;
use super::schema::ledger;
use super::schema::company;
use super::schema::industry;

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
  pub alpha: f64,
  pub stdev: f64,
}

#[derive(Insertable)]
#[table_name="company"]
pub struct NewCompany {
  pub name: String,
  pub dividend: NaiveDate,
  pub shares: i32,
  pub alpha: f64,
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