use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod company;
pub mod industry;
pub mod ledger;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  return PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url));
}

pub fn clear(conn: &PgConnection) {
  industry::clear(conn);
  company::clear(conn);
  ledger::clear(conn);
}