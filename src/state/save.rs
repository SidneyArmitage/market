use std::fs::File;
use std::io::prelude::*;


use super::section;


pub fn save (address: &str, object: section::Root) {
  let mut file = File::open(address)
    .expect("File does not exist");
  file.write_all(section::root::save(&object).as_bytes()).expect("did not write file");
}