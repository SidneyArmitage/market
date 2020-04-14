extern crate market;
extern crate diesel;

use std::fs;
use std::env;

use self::market::*;


fn main() {
  let connection = establish_connection();
  let filename = env::var("INDUSTRY_FILE")
    .expect("INDUSTRY_FILE must be set");
  let contents = fs::read_to_string(filename)
    .expect("Unable to read file");
  let mut lines = contents.lines();
  lines.next();
  clear_industry(&connection);
  for line in lines {
      let elements: Vec<&str> = line.split("\t").collect();
      println!("{}", line);
      let name = String::from(elements[0]);
      let beta = elements[1].parse::<f64>().unwrap();
      let stdev = elements[2].parse::<f64>().unwrap();
      let _company = create_industry(&connection, name, beta, stdev);
  }
  // load industries from config file
  // load company names
  // generate companies
}