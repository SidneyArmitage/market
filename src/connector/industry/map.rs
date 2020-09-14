
use diesel::pg::PgConnection;
use diesel::prelude::*;

use super::super::super::model::IndustryMap;
use super::super::super::schema::industry_map;

pub fn create (conn: &PgConnection, industry: i32, company: i32, beta: f64, weight: f64) {
  let new_industry_map = IndustryMap {
    industry: industry,
    company: company,
    beta: beta,
    weight: weight,
  };
  
  diesel::insert_into(industry_map::table)
    .values(&new_industry_map)
    .execute(conn)
    .expect("Error saving new industry mapping");
}

pub fn fetch_from_company(conn: &PgConnection, id: i32) -> std::vec::Vec<IndustryMap> {
  use super::super::super::schema::industry_map::dsl::*;
  return industry_map
    .filter(company.eq(id))
    .load::<IndustryMap>(conn)
    .expect("unable to fetch industry");
}