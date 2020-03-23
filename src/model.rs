use diesel::data_types::Cents;
use super::schema::ledger;
use super::schema::company;

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
}

#[derive(Insertable)]
#[table_name="company"]
pub struct NewCompany {
  pub name: String
}