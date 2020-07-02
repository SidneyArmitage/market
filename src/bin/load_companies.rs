extern crate market;
extern crate diesel;
extern crate rand;

use std::env;
use rand::Rng;
use chrono::NaiveDate;

use self::market::*;
use self::market::util::*;

fn main() {
  let mut rng = rand::thread_rng();
  let connection = establish_connection();
  // load industries
  let mut industries = fetch_industries(&connection);
  let amount = industries.len();
  let mut gaussian = market::util::random::create_gaussian();
  let close = 3.090;
  let deviation = 0.5 / close;
  // split companies between each industry
  for x in 65u8..90u8 {
    for y in 65u8..90u8 {
      for z in 65u8..90u8 {
        let initialDividend = NaiveDate::from_num_days_from_ce(rng.gen_range(0, 365));
        let company = create_company(&connection, format!("{}{}{}", x as char, y as char, z as char), rng.gen_range(100_000, 100_000_000), gaussian.generate(1.0, 0.25), initialDividend);
        // initial dividend
        create_dividend(&connection, company.id, initialDividend, gaussian.generate(0.1f64, 0.1f64));
        let mut weights: Vec<f64> = vec![0f64; amount];
        let mut sum: f64 = 0f64;
        // link segments
        for i in 0..amount {
          let mut weight = gaussian.generate(0f64, deviation);
          if weight < 0.0 {
            weight = 1f64 - weight;
          }
          sum += weight;
          weights.push(weight);
        }
        for i in weights {
          let beta = gaussian.generate(1.04, 0.1);
          let weight = i / sum;
        }
      }
    }
  }
  // create link for industry map
}