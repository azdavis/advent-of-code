use crate::intcode::{parse, Intcode};

pub fn p1(s: &str) -> i64 {
  go(parse(s).collect(), 12, 2)
}

pub fn p2(s: &str) -> i64 {
  let p: Vec<_> = parse(s).collect();
  for noun in 0..=99 {
    for verb in 0..=99 {
      if go(p.clone(), noun, verb) == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  panic!("no solution")
}

fn go(mut ns: Vec<i64>, noun: i64, verb: i64) -> i64 {
  ns[1] = noun;
  ns[2] = verb;
  let mut p = Intcode::new(ns);
  let mut output = Vec::new();
  assert!(p.run(&mut output).is_done());
  assert!(output.is_empty());
  p.read_zeroth()
}

#[test]
fn t() {
  let inp = include_str!("input/d02.txt");
  assert_eq!(p1(inp), 11590668);
  assert_eq!(p2(inp), 2254);
}
