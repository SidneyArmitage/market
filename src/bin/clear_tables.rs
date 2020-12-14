extern crate diesel;
extern crate market;

use market::connector::{establish_connection, clear};

fn main() {
  let connection = establish_connection();
  clear(&connection);
}