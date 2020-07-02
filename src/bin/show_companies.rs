extern crate market;
extern crate diesel;

use self::market::*;
use self::model::*;
use diesel::prelude::*;
use diesel::dsl::count_star;
use std::io::{stdin};

fn main() {
  use market::schema::company::dsl::*;

  let connection = connector::establish_connection();
  let mut input = String::new();
  let mut count = 0;
  let length = company.select(count_star())
    .first(&connection)
    .expect("Error loading count");
  println!("Companies");
  println!("");
  println!("length: {}", length);
  println!("");
  println!("----------|------------------");
  println!("    ID    |       Name       ");
  println!("----------|------------------");
  
  let mut next = count != length;
  while next {
    let result = company.offset(count)
      .limit(1)
      .load::<Company>(&connection)
      .expect("Error loading companies");
    for entity in result {
      println!("{}|{}", format!("{: ^10}", entity.id), format!("{: ^18}", entity.name));
    }
    stdin().read_line(&mut input).unwrap();
    count += 1;
    next = input.len() == 0 && count != length;
  }
  println!("----------|------------------");
}