use chrono::{NaiveDate, Datelike};

pub struct State {
  date: i32,
}

pub mod load;
mod section;

impl State {

  pub fn get_date (&mut self) -> NaiveDate {
    return NaiveDate::from_num_days_from_ce(self.date);
  }

  pub fn set_date (&mut self, date: NaiveDate) {
    self.date = date.num_days_from_ce();
  }

}