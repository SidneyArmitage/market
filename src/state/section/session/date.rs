use super::super::{Root, root, Loop, session};

pub fn load (value: &str, object: &mut Root) -> Loop {
  object.session.date = value.parse::<i32>().expect("Date not an integer");
  return Loop {
    functions: vec![Box::new(session::load as fn (&str, &str, &mut Root) -> Loop), Box::new(root::load as fn (&str, &str, &mut Root) -> Loop)]
  };
}

pub fn save (value: i32) -> String {
  return format!("\t\tdate: {}\n", value);
}