use std::collections::{HashMap, HashSet, VecDeque};

pub fn p1(s: &str) -> usize {
  let inp = parse(s);
  let mut visited = HashSet::new();
  let mut queue = VecDeque::from(vec!["COM"]);
  let mut level = 0;
  let mut ret = 0;
  loop {
    let level_len = queue.len();
    if level_len == 0 {
      break;
    }
    for _ in 0..level_len {
      let vtx = queue.pop_front().unwrap();
      if !visited.insert(vtx) {
        continue;
      }
      ret += level;
      queue.extend(inp.get(vtx).iter().copied().flatten());
    }
    level += 1;
  }
  ret
}

pub fn p2(s: &str) -> usize {
  todo!()
}

type Input<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse(s: &str) -> Input<'_> {
  let mut ret = Input::new();
  for line in s.split('\n') {
    if line.is_empty() {
      continue;
    }
    let mut parts = line.split(')');
    let center = parts.next().unwrap();
    let orbiter = parts.next().unwrap();
    assert!(parts.next().is_none());
    ret.entry(center).or_default().insert(orbiter);
  }
  ret
}

#[test]
fn t() {
  let inp = include_str!("input/d06.txt");
  assert_eq!(p1(inp), 150150);
  // assert_eq!(p2(inp), ___);
}
