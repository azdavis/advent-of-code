use std::convert::TryInto as _;

pub fn p1(s: &str) -> i32 {
  let ns = parse(s);
  let mut output = Vec::new();
  go(ns, &[1], &mut output);
  let code = output.pop().unwrap();
  for x in output {
    assert_eq!(x, 0);
  }
  code
}

pub fn p2(s: &str) -> i32 {
  todo!()
}

fn go(mut ns: Vec<i32>, input: &[i32], output: &mut Vec<i32>) {
  let mut input = input.iter().copied();
  let mut idx = 0;
  loop {
    let cur = ns[idx];
    let op = cur % 100;
    let modes = cur / 100;
    match op {
      1 => {
        let (a, b, c) = get_3_arg(&ns, idx, modes);
        ns[c] = a + b;
        idx += 4;
      }
      2 => {
        let (a, b, c) = get_3_arg(&ns, idx, modes);
        ns[c] = a * b;
        idx += 4;
      }
      3 => {
        let a = get_pos_arg(&ns, idx, 1, modes);
        ns[a] = input.next().unwrap();
        idx += 2;
      }
      4 => {
        let a = get_arg(&ns, idx, 1, modes);
        output.push(a);
        idx += 2;
      }
      99 => break,
      _ => panic!("invalid op: {}", op),
    }
  }
}

fn get_arg(ns: &[i32], idx: usize, off: usize, modes: i32) -> i32 {
  let val = ns[idx + off];
  match get_mode(off, modes) {
    Mode::Position => ns[u(val)],
    Mode::Immediate => val,
  }
}

fn get_pos_arg(ns: &[i32], idx: usize, off: usize, modes: i32) -> usize {
  assert!(matches!(get_mode(off, modes), Mode::Position));
  u(ns[idx + off])
}

fn get_3_arg(ns: &[i32], idx: usize, modes: i32) -> (i32, i32, usize) {
  let a = get_arg(&ns, idx, 1, modes);
  let b = get_arg(&ns, idx, 2, modes);
  let c = get_pos_arg(ns, idx, 3, modes);
  (a, b, c)
}

fn get_mode(off: usize, modes: i32) -> Mode {
  let div = (1..off).fold(1, |ac, _| ac * 10);
  match (modes / div) % 10 {
    0 => Mode::Position,
    1 => Mode::Immediate,
    mode => panic!("invalid mode: {}", mode),
  }
}

enum Mode {
  Position,
  Immediate,
}

fn u(n: i32) -> usize {
  n.try_into().unwrap()
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
  // assert_eq!(p2(inp), ___);
}
