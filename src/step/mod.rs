use diesel::pg::PgConnection;
use chrono::NaiveDate;
use std::collections::HashMap;

use super::connector::industry::;
use super::connector::company;
use super::connector::dividend;
use super::util::random;

pub struct Input {
  conn: &PgConnection,
  last_day: NaiveDate,
  new_date: NaiveDate,
  rng: &mut GenerateGaussian,
  scale: f64,
}

pub fn industries(args: Input) -> HashMap<i32, f64> {
  // get market
  let delta_map HashMap<i32, f64> = HashMap::new();
  let market_id = industry::fetch_total(args.conn).id;
  let market_original_value = industry::value::fetch(args.conn, market_id, args.last_date).value;
  // set market - next date
  let market_new_value = walk(args.rng, args.scale, market_original_value);
  industry::value::create(args.conn, market_id, args.new_date, market_new_value);
  let market_delta = market_new_value - market_original_value;
  delta_map.insert(market_id, market_delta);
  // get every industry
  for c in fetch_all(args.conn) {
    let current_value = industry::value::fetch(args.conn, c.id, args.last_day).value;
    let new_value = c.beta * (market_delta) + walk(args.rng, args.scale, current_value);
    industry::value::create(args.conn, c.id, args.new_date, new_value);
    delta_map.insert(new_value - current_value);
  }
  return delta_map;
}

pub fn companies (args: Input, delta_map: &HashMap<i32, f64>) {
  // for each company
  for comp in company::fetch_all(args.conn) {
    let current_value = company::dividend::projected::fetch(args.conn, comp.id, args.last_day).index;
    let dividend = company::dividend::fetch(args.conn, comp.id, comp.dividend);
    let industry_map = industry::map::fetch_from_company(args.conn, comp.id);
    let mut new_value = walk(args.rng, args.scale, current_value);
    for i in industry_map {
      let new_value = new_value + match delta_map.get(i) {
        None => 0,
        Some(delta) => delta * i.beta * i.weight,
      }
    }
    company::dividend::projected::create(args.conn, comp.id, args.new_date, new_value);
    if args.new_date === comp.dividend {

    } else if args.new_date < dividend.announcement_date {

    } else {

    }
  }
}

fn walk (rng: &mut GenerateGaussian, scale: f64, value: f64) -> f64 {
  return rng.generate(1f64, scale) * value;
}