use crate::intcode::{parse, Intcode};

pub fn p1(s: &str) -> i64 {
  let mut p = Intcode::new(parse(s));
  p.input(1);
  let mut output = Vec::with_capacity(1);
  assert!(p.run(&mut output).is_done());
  let out = output.pop().unwrap();
  assert!(output.is_empty());
  out
}

pub fn p2(s: &str) -> i64 {
  todo!()
}

#[test]
fn t() {
  let inp = include_str!("input/d09.txt");
  assert_eq!(p1(inp), 2204990589);
  // assert_eq!(p2(inp), ___);
}
