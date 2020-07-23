use super::super::{Root, root, Loop, option};

#[derive(Copy, Clone)]
pub enum Step {
  Daily,
  Weekly,
  Monthly,
  SemiAnnually,
  Annually,
}

pub fn load (value: &str, object: &mut Root) -> Loop {
  match value {
    "Daily" => object.option.step = Step::Daily,
    "Weekly" => object.option.step = Step::Weekly,
    "Monthly" => object.option.step = Step::Monthly,
    "SemiAnnually" => object.option.step = Step::SemiAnnually,
    "Annually" => object.option.step = Step::Annually,
    _ => panic!("Invalid step given"),
  };
  return Loop {
    functions: vec![
      Box::new(option::load as fn (&str, &str, &mut Root) -> Loop),
      Box::new(root::load as fn (&str, &str, &mut Root) -> Loop),
    ],
  };
}

pub fn save (value: Step) -> String {
  return format!("\t\tstep: {} \n", match value {
    Step::Daily => "Daily",
    Step::Weekly => "Weekly",
    Step::Monthly => "Monthly",
    Step::SemiAnnually => "SemiAnnually",
    Step::Annually => "Annually",
  });
}