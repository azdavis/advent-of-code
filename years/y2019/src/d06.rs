use helpers::dijkstra::{dijkstra, Graph as _, MapGraph};
use helpers::HashSet;
use std::collections::VecDeque;

pub fn p1(s: &str) -> usize {
  let inp = parse(s, |inp, center, orbiter| {
    inp.entry(center).or_default().insert(orbiter);
  });
  // bfs
  let mut visited = HashSet::default();
  let mut queue = VecDeque::from(vec!["COM"]);
  let mut level = 0;
  let mut ret = 0;
  while !queue.is_empty() {
    for _ in 0..queue.len() {
      let vtx = queue.pop_front().unwrap();
      if !visited.insert(vtx) {
        continue;
      }
      ret += level;
      queue.extend(inp.neighbors(vtx));
    }
    level += 1;
  }
  ret
}

pub fn p2(s: &str) -> usize {
  let inp = parse(s, |inp, center, orbiter| {
    inp.entry(center).or_default().insert(orbiter);
    inp.entry(orbiter).or_default().insert(center);
  });
  dijkstra(&inp, "YOU", "SAN").unwrap() - 2
}

fn parse(s: &str, add: for<'a> fn(&mut MapGraph<&'a str>, &'a str, &'a str)) -> MapGraph<&str> {
  let mut ret = MapGraph::default();
  for line in s.lines() {
    let mut parts = line.split(')');
    let center = parts.next().unwrap();
    let orbiter = parts.next().unwrap();
    assert!(parts.next().is_none());
    add(&mut ret, center, orbiter);
  }
  ret
}

#[test]
fn t() {
  let s = include_str!("input/d06.txt");
  assert_eq!(p1(s), 150150);
  assert_eq!(p2(s), 352);
}
