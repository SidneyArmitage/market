extern crate market;
extern crate diesel;


use market::initialize::companies;
use market::connector::establish_connection;

fn main() {
  let connection = establish_connection();
  companies(&connection);
  
  // create link for industry map
}