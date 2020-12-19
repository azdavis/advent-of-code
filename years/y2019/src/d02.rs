use crate::intcode::{parse, Intcode};

pub fn p1(s: &str) -> i32 {
  go(parse(s), 12, 2)
}

pub fn p2(s: &str) -> i32 {
  let p = parse(s);
  for noun in 0..=99 {
    for verb in 0..=99 {
      if go(p.clone(), noun, verb) == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  panic!("no answer exists")
}

fn go(mut ns: Vec<i32>, noun: i32, verb: i32) -> i32 {
  ns[1] = noun;
  ns[2] = verb;
  let mut p = Intcode::new(ns);
  let mut output = Vec::new();
  assert!(p.run(&mut output).is_done());
  assert!(output.is_empty());
  p.into_mem().read(0)
}

#[test]
fn t() {
  let inp = include_str!("input/d02.txt");
  assert_eq!(p1(inp), 11590668);
  assert_eq!(p2(inp), 2254);
}
