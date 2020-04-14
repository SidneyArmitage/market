extern crate market;
extern crate diesel;
extern crate rand;

use std::fs;
use std::env;
use rand::Rng;

use self::market::*;

fn main() {
  let mut rng = rand::thread_rng();
  let connection = establish_connection();
  // load names
  let filename = env::var("COMPANY_FILE")
  .expect("COMPANY_FILE must be set");
  let contents = fs::read_to_string(filename)
    .expect("Unable to read file");
  let names: Vec<&str>  = contents.split("\n\r").collect();
  // load industries
  let mut industries = fetch_industries(&connection);
  let amount= names.len() / industries.len();
  let mut current = industries.pop();
  let mut count = 0;
  // split companies between each industry
  for name in names {
    create_company(&connection, name.to_string(), rng.gen_range(100_000, 100_000_000), 0.0, 0.0);
    count += 1;
    if(amount == count) {
      current = industries.pop();
    }
  }
  // random alpha
  // random std dev
  // create dividend
  // create link for industry map
}