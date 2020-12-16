
use diesel::pg::PgConnection;
use std::{fs, env};
use chrono::NaiveDate;
use rand::Rng;
use super::model::IndustryMap;

use super::connector::industry;
use super::connector::company;
use super::util::random;

// initialize industries
pub fn industries(conn: &PgConnection) {
  let filename = env::var("INDUSTRY_FILE")
    .expect("INDUSTRY_FILE must be set");
  let contents = fs::read_to_string(filename)
    .expect("Unable to read file");
  let mut lines = contents.lines();
  lines.next();
  industry::clear(conn);
  for line in lines {
      let elements: Vec<&str> = line.split("\t").collect();
      let zero = NaiveDate::from_num_days_from_ce(0);
      print!("{}\r", line);
      let name = String::from(elements[0]);
      let beta = elements[1].parse::<f64>().unwrap();
      let stdev = elements[2].parse::<f64>().unwrap();
      let sector = industry::create(conn, name, beta, stdev);
      industry::value::create(conn, sector.id, zero, 0f64);
  }
}

// initialize companies
pub fn companies(conn: &PgConnection) {
  let mut rng = rand::thread_rng();
  // load industries
  let industries = industry::fetch_all(&conn);
  let amount = industries.len();
  let mut gaussian = random::create_gaussian();
  let close = 3.090;
  let deviation = 0.5 / close;
  let zero = NaiveDate::from_num_days_from_ce(0);
  let mut company_map = Vec::new();
  // split companies between each industry
  for x in 65u8..90u8 {
    for y in 65u8..90u8 {
      for z in 65u8..90u8 {
        print!("initialized company {}{}{}\r", x as char, y as char, z as char);
        let initial_dividend = NaiveDate::from_num_days_from_ce(rng.gen_range(0, 365));
        let current = company::create(conn, format!("{}{}{}", x as char, y as char, z as char), rng.gen_range(100_000, 100_000_000), gaussian.generate(1.0, 0.25), initial_dividend);
        // initial dividend
        company::dividend::create(conn, current.id, initial_dividend, zero, zero, gaussian.generate(0.1f64, 0.1f64));
        let mut weights: Vec<f64> = vec![0f64; amount];
        let mut sum: f64 = 0f64;
        // link segments
        for _ in 0..amount {
          let mut weight = gaussian.generate(0f64, deviation);
          if weight < 0.0 {
            weight = 1f64 - weight;
          }
          sum += weight;
          weights.push(weight);
        }
        for i in 0..amount {
          let beta = gaussian.generate(1.04, 0.1);
          // could be removed for optimisation
          company_map.push(IndustryMap {
            industry: industries.get(i).expect("Missing Industry").id,
            company: current.id,
            beta: beta,
            weight: weights.get(i).expect("Missing weight") / sum,
          });
        }
        industry::map::create(conn, &company_map);
        company_map.clear();
      }
    }
  }
}