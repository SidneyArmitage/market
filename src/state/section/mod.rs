pub mod session;

pub struct Root {
  pub session: session::Session,
}

pub struct Loop {
  pub functions: Vec<Box<fn (&str, &str, &mut Root) -> Loop>>,
}

pub fn init () -> Root {
  return Root {
    session: session::init(),
  };
}

pub fn handle_unexpected (path: &str, id: &str, value: &str, functions: Loop) -> Loop {
  println!("Unexpected configuration");
  println!(" at {} for {} with value of {}", path, id, value);
  return functions;
}

pub mod root {
  use super::{Loop, handle_unexpected, Root};
  use super::session;

  pub fn load (id: &str, value: &str, _object: &mut Root) -> Loop {
    return match id {
      "save" => Loop {
        functions: vec![Box::new(session::load as fn (&str, &str, &mut Root) -> Loop)],
      },
      _ => handle_unexpected("root", id, value, Loop {
        functions: vec![Box::new(load as fn (&str, &str, &mut Root) -> Loop)]
      }),
    };
  }

  pub fn save (value: & Root) -> String {
    return format!("{}", session::save(&value.session));
  }
}