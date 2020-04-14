extern crate market;
extern crate diesel;

use self::market::*;
use std::io::{stdin};

fn main() {
  let connection = establish_connection();
  println!("Company Name:");
  let mut name = String::new();
  stdin().read_line(&mut name).unwrap();
  let name = name.trim_end();
  let _company = create_company(&connection, name.to_string(), 0, 0.0, 0.0);
}