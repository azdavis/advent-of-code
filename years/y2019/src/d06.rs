use helpers::dijkstra::{dijkstra, Graph};
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
      queue.extend(inp.get(vtx).into_iter().flatten());
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

fn parse(
  s: &str,
  add: for<'a> fn(&mut Graph<&'a str>, &'a str, &'a str),
) -> Graph<&str> {
  let mut ret = Graph::default();
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
