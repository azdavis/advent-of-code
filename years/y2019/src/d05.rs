use crate::intcode::Intcode;

pub fn p1(s: &str) -> i64 {
  let mut p = Intcode::parse(s);
  let mut output = Vec::with_capacity(2);
  p.input(1);
  assert!(p.run(&mut output).is_done());
  let code = output.pop().unwrap();
  for x in output {
    assert_eq!(x, 0);
  }
  code
}

pub fn p2(s: &str) -> i64 {
  let mut p = Intcode::parse(s);
  let mut output = Vec::with_capacity(1);
  p.input(5);
  assert!(p.run(&mut output).is_done());
  let code = output.pop().unwrap();
  assert!(output.is_empty());
  code
}

#[test]
fn t() {
  let s = include_str!("input/d05.txt");
  assert_eq!(p1(s), 13210611);
  assert_eq!(p2(s), 584126);
}
