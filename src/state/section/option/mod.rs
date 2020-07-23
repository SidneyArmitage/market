use super::{Root, Loop, handle_unexpected, root};

pub mod step;
pub mod volatility;

pub struct Option {
  pub step: step::Step,
  pub volatility: f64,
}

pub fn init() -> Option {
  return Option {
    volatility: 0.1,
    step: step::Step::Monthly,
  };
}

pub fn load (id: &str, value: &str, object: &mut Root) -> Loop {
  return match id {
    "step" => step::load(value, object),
    "volatility" => volatility::load(value, object),
    _ => handle_unexpected("root/option", id, value, Loop {
        functions: vec![Box::new(load as fn (&str, &str, &mut Root) -> Loop), Box::new(root::load as fn (&str, &str, &mut Root) -> Loop)],
      },),
  };
}

pub fn save (value: &Option) -> String {
  return format!("\tsession:\n{}\n{}", step::save(value.step), volatility::save(value.volatility));
}