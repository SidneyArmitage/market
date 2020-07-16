use std::fs::File;
use super::section;


pub fn save (address: &str) -> std::io::Result<()> {
  let mut file = File::open(address)
    .expect("File does not exist");
  file.write_all(section::root::save);
}