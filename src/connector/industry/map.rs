
use diesel::pg::PgConnection;
use diesel::prelude::*;

use super::super::super::model::IndustryMap;
use super::super::super::schema::industry_map;

pub fn create (conn: &PgConnection, map: &Vec<IndustryMap>) {
  
  diesel::insert_into(industry_map::table)
    .values(map)
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

pub fn clear(conn: &PgConnection) {
  use super::super::super::schema::industry_map::dsl::*;
  println!("cleared {} from map", diesel::delete(industry_map).execute(conn).expect("Unable to clear map"));
}