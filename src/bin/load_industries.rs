extern crate market;
extern crate diesel;


use market::initialize::industries;
use market::connector::establish_connection;


fn main() {
  let connection = establish_connection();
  industries(&connection);
}