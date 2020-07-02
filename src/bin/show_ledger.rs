extern crate market;
extern crate diesel;

use self::market::*;
use self::model::*;
use diesel::prelude::*;
use diesel::dsl::count_star;
use diesel::data_types::Cents;
use std::io::{stdin};

fn main() {
  use market::schema::*;
  // use market::schema::ledger::dsl::*;
  // use market::schema::company::dsl::*;
  
  let connection = connector::establish_connection();
  let mut input = String::new();
  let mut count_c = 0;
  let length_c = company::dsl::company.select(count_star())
    .first(&connection)
    .expect("Error loading count");
  println!("Ledger");
  println!("");
  println!("------------------|-----|---------|------------------");
  println!("      Company     | Buy |  Price  |     Quantity     ");
  println!("------------------|-----|---------|------------------");
  
  let mut next = count_c != length_c;
  while next {
    // get all companies
    let result_c = company::dsl::company.offset(count_c)
      .limit(1)
      .load::<Company>(&connection)
      .expect("Error loading companies");
    let mut ran = false;
    for entity in result_c {
      let name_c = format!("{: ^18}", entity.name);
      let id = entity.id;
      let mut count_l = 0;
      let length_l = ledger::dsl::ledger.filter(ledger::dsl::company.eq(id))
        .select(count_star())
        .first(&connection)
        .expect("Error loading count");
      next = count_l != length_l;
      while next {
        // get all ledger entries
        let result_l = ledger::dsl::ledger.filter(ledger::dsl::company.eq(id))
          .offset(count_l)
          .limit(1)
          .load::<Entry>(&connection)
          .expect("Error loading companies");
        for entry in result_l {
          let Cents(cents) = entry.price;
          println!("{}|{}|{}|{}", name_c, format!("{: ^5}", entry.is_buy), format!("{: ^10}", cents), format!("{: ^10}", entry.quantity));
        }
        stdin().read_line(&mut input).unwrap();
        println!("length company: {}, ledger: {}", length_c, length_l);
        println!("counts company: {}, ledger: {}", count_c, count_l);
        count_l += 1;
        ran = true;
        next = input.len() == 0 && count_l != length_l;
      }
    }
    count_c += 1;
    next = count_c != length_c && (ran == false || next);
  }
  println!("------------------|-----|---------|------------------");
}