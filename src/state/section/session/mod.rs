use super::{Root, handle_unexpected, root, Loop};
pub mod date;


pub struct Session {
  pub date: i32,
}

pub fn init() -> Session {
  return Session {
    date: 0,
  };
}

pub fn load (id: &str, value: &str, object: &mut Root) -> Loop {
  return match id {
    "date" => date::load(value, object),
    _ => handle_unexpected("root/session", id, value, Loop {
      functions: vec![Box::new(load as fn (&str, &str, &mut Root) -> Loop), Box::new(root::load as fn (&str, &str, &mut Root) -> Loop)]
    }),
  };
}

pub fn save (value: &Session) -> String {
  return format!("\tsession:\n{}", date::save(value.date));
}