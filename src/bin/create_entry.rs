extern crate market;
extern crate diesel;

use diesel::data_types::Cents;
use self::market::*;

fn main() {
  let connection = establish_connection();
  let _entry = create_entry(&connection, 1, true, Cents(123_456), 1, 0);
}