use super::super::{Root, root, Loop, option};

pub fn load (value: &str, object: &mut Root) -> Loop {
  object.option.volatility = value.parse::<f64>().expect("volatility not a float");
  return Loop {
    functions: vec![
      Box::new(option::load as fn (&str, &str, &mut Root) -> Loop), Box::new(root::load as fn (&str, &str, &mut Root) -> Loop),
      ],
  };
}

pub fn save (value: f64) -> String {
  return format!("\t\tvolatility: {}\n", value);
}