use helpers::HashMap;
use std::cmp::Reverse;

fn run(s: &str) -> (usize, usize) {
  let mut ns: Vec<u8> = s
    .trim()
    .split_ascii_whitespace()
    .map(|it| it.parse().unwrap())
    .collect();
  let mut map = HashMap::<Vec<u8>, usize>::default();
  let mut cur = 0usize;
  loop {
    if let Some(prev) = map.insert(ns.clone(), cur) {
      return (prev, cur);
    }
    let (mut idx, &n) = ns
      .iter()
      .enumerate()
      .max_by_key(|&(idx, &n)| (n, Reverse(idx)))
      .unwrap();
    ns[idx] = 0;
    for _ in 0..n {
      idx = (idx + 1) % ns.len();
      ns[idx] += 1;
    }
    cur += 1;
  }
}

pub fn p1(s: &str) -> usize {
  let (_, cur) = run(s);
  cur
}

pub fn p2(s: &str) -> usize {
  let (prev, cur) = run(s);
  cur - prev
}

#[test]
fn t() {
  let s = include_str!("input/d06.txt");
  assert_eq!(p1(s), 14029);
  assert_eq!(p2(s), 2765);
}

#[test]
fn ex1() {
  let s = "0 2 7 0";
  assert_eq!(p1(s), 5);
  assert_eq!(p2(s), 4);
}
