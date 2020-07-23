extern crate market;


fn main () {
  market::state::save::save("./config/store.yml", market::state::load::load(""));
}