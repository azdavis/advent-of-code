use crate::intcode::Intcode;
use std::io::Write as _;

fn run(real: bool, prog: &str, commands: &[u8]) -> usize {
  let mut p = Intcode::parse(prog);
  let mut out = Vec::new();
  assert!(p.run(&mut out).needs_input());
  out.clear();
  for &b in commands {
    p.input(b.into());
  }
  assert!(p.run(&mut out).is_done());
  if real {
    let &n = out.last().unwrap();
    n.try_into().unwrap()
  } else {
    let buf: Vec<u8> = out.iter().map(|&n| n.try_into().unwrap()).collect();
    std::io::stdout().write_all(&buf).unwrap();
    0
  }
}

pub fn p1(s: &str) -> usize {
  // (A and B and C).not and D
  let inp = br"
NOT T T
AND A T
AND B T
AND C T
NOT T T
AND D T
NOT T T
NOT T J
WALK
";
  run(true, s, &inp[1..])
}

pub fn p2(s: &str) -> usize {
  // (((A and B and C).not and D and H).not and A).not
  let inp = br"
NOT T T
AND A T
AND B T
AND C T
NOT T T
AND D T
AND H T
NOT T T
AND A T
NOT T J
RUN
";
  run(true, s, &inp[1..])
}

#[test]
fn t() {
  let s = include_str!("input/d21.txt");
  assert_eq!(p1(s), 19_352_864);
  assert_eq!(p2(s), 1_142_488_337);
}
