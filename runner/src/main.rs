use std::io::{stdin, Read as _};

fn main() {
  let mut inp = String::new();
  stdin().read_to_string(&mut inp).unwrap();
  println!("{}", y2020::d09::p2(&inp));
}
