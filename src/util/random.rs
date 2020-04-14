extern crate rand;

use rand::Rng;
use rand::rngs::*;
use std::f64;

pub struct GenerateGassian {
  spare: f64,
  rng: ThreadRng,
}

impl GenerateGassian {

  pub fn generate(&mut self, mean: f64, std: f64) ->f64 {
    if self.spare != 0.0 {
      let out = self.spare * std + mean;
      self.spare = 0.0;
      return out;
    }
    let mut u: f64 = 0.0;
    let mut v: f64 = 0.0;
    let mut s: f64 = 0.0;
    while  s>= 1.0 || s == 0.0 {
      u = self.rng.gen::<f64>() * 2.0 - 1.0; // assuming 0 < rnd < 1
      v = self.rng.gen::<f64>() * 2.0 - 1.0; // assuming 0 < rnd < 1
      s = u * u + v * v;
    }
    s = (-2.0 * s.ln() / s).sqrt();
    self.spare = v * s;
    return mean + std * u * s;
  }
}

#[test]
fn test_Nrand() {
  let mut rng = rand::thread_rng();

  let mut gen = GenerateGassian {
    spare: 0.0,
    rng: rng,
  };

  for n in 1..200 {
    println!("{}", gen.generate(0.0, 0.25));
  }
}