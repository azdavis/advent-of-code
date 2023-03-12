use crate::intcode::Intcode;

pub fn p1(s: &str) -> i64 {
  go(s, 1)
}

pub fn p2(s: &str) -> i64 {
  go(s, 2)
}

fn go(s: &str, inp: i64) -> i64 {
  let mut p = Intcode::parse(s);
  p.input(inp);
  let mut output = Vec::with_capacity(1);
  assert!(p.run(&mut output).is_done());
  let out = output.pop().unwrap();
  assert!(output.is_empty());
  out
}

#[test]
fn t() {
  let s = include_str!("input/d09.txt");
  assert_eq!(p1(s), 2_204_990_589);
  assert_eq!(p2(s), 50008);
}
