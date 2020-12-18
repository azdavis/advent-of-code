use crate::intcode::run;

pub fn p1(s: &str) -> i32 {
  let ns = parse(s);
  // we output a code and run at least one test
  let mut output = Vec::with_capacity(2);
  run(ns, &[1], &mut output);
  let code = output.pop().unwrap();
  for x in output {
    assert_eq!(x, 0);
  }
  code
}

pub fn p2(s: &str) -> i32 {
  let ns = parse(s);
  // just output the code
  let mut output = Vec::with_capacity(1);
  run(ns, &[5], &mut output);
  let code = output.pop().unwrap();
  assert!(output.is_empty());
  code
}

fn parse(s: &str) -> Vec<i32> {
  s.split('\n')
    .next()
    .unwrap()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect()
}

#[test]
fn t() {
  let inp = include_str!("input/d05.txt");
  assert_eq!(p1(inp), 13210611);
  assert_eq!(p2(inp), 584126);
}
