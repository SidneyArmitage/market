use std::fs::File;
use std::io::{self, BufRead};

use super::section;


pub fn load (address: &str) -> section::Root {
  let file = File::open(address)
    .expect("File does not exist");
  let mut root = section::init();
  let lines_wrapper = io::BufReader::new(file)
    .lines();
  let mut lines: Vec<String> = vec![];
  for line in lines_wrapper {
    lines.push(line.unwrap())
  }
  read(&mut lines.into_iter().rev().collect(), 0, &mut root, section::Loop {
    functions: vec![Box::new(section::root::load as fn (&str, &str, &mut section::Root) -> section::Loop)],
  });
  return root;
}

fn count_space (element: &str) -> usize {
  let mut count: usize = 0;
  for c in element.chars() {
    if c == ' ' {
      count += 1;
    } else if c == '\t' {
      count += 2;
    } else {
      return count / 2;
    }
  }
  return count;
}

fn read (lines: &mut Vec<String>, spaces: usize, object: &mut section::Root, handlers: section::Loop) {
  let line = lines.pop().unwrap();
  let current_spaces = count_space(&line);
  let trimmed = line.trim();
  let split: Vec<&str> = trimmed.splitn(2, ':').collect();
  if lines.len() > 0 {
    let handle = handlers.functions[spaces - current_spaces](split[0], split[1], object);
    read(lines, current_spaces, object, handle);
  }
}

#[test]
fn test_read() {
  let obj = section::init();
  read([
    "save:",
    "  date: 1"
  ], 0, &obj, section::root);
  assert_eq!(obj.save.date, 1)
}