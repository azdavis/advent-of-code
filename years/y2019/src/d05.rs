use crate::intcode::{parse, Intcode};

pub fn p1(s: &str) -> i32 {
  let mut p = Intcode::new(parse(s));
  let mut output = Vec::with_capacity(2);
  p.input(1);
  assert!(p.run(&mut output).is_done());
  let code = output.pop().unwrap();
  for x in output {
    assert_eq!(x, 0);
  }
  code
}

pub fn p2(s: &str) -> i32 {
  let mut p = Intcode::new(parse(s));
  let mut output = Vec::with_capacity(1);
  p.input(5);
  assert!(p.run(&mut output).is_done());
  let code = output.pop().unwrap();
  assert!(output.is_empty());
  code
}

#[test]
fn t() {
  let inp = include_str!("input/d05.txt");
  assert_eq!(p1(inp), 13210611);
  assert_eq!(p2(inp), 584126);
}
