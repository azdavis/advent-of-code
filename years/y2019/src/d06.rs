use helpers::infinitable::Infinitable;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub fn p1(s: &str) -> usize {
  let inp = parse(s, |inp, center, orbiter| {
    inp.entry(center).or_default().insert(orbiter);
  });
  // bfs
  let mut visited = HashSet::new();
  let mut queue = VecDeque::from(vec!["COM"]);
  let mut level = 0;
  let mut ret = 0;
  loop {
    if queue.is_empty() {
      break;
    }
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

/// dijkstra's algorithm. don't need to store predecessors info (cf wikipedia).
fn dijkstra<T>(graph: &Graph<T>, start: T, end: T) -> Option<usize>
where
  T: Hash + Ord + Copy,
{
  let mut distances: HashMap<_, _> = graph
    .keys()
    .map(|&node| (node, Infinitable::PosInf))
    .collect();
  distances.insert(start, Infinitable::Finite(0));
  let mut pq: BinaryHeap<_> = distances
    .iter()
    .map(|(&node, &dist)| Elem::new(node, dist))
    .collect();
  while let Some(u) = pq.pop() {
    let u_dist = distances[&u.node];
    if u.node == end {
      match u_dist {
        Infinitable::Finite(x) => return Some(x),
        _ => unreachable!(),
      }
    }
    if u.dist.0 > u_dist {
      continue;
    }
    let new_dist = u_dist + 1;
    for &node in graph.get(&u.node).into_iter().flatten() {
      if new_dist >= distances[&node] {
        continue;
      }
      distances.insert(node, new_dist);
      pq.push(Elem::new(node, new_dist));
    }
  }
  None
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Elem<T> {
  dist: Reverse<Infinitable<usize>>,
  node: T,
}

impl<T> Elem<T> {
  fn new(node: T, dist: Infinitable<usize>) -> Self {
    Self {
      node,
      dist: Reverse(dist),
    }
  }
}

#[test]
fn elem_cmp() {
  let a = Elem {
    dist: Reverse(Infinitable::PosInf),
    node: "a",
  };
  let b = Elem {
    dist: Reverse(Infinitable::Finite(3)),
    node: "a",
  };
  assert!(a < b);
}

type Graph<T> = HashMap<T, HashSet<T>>;

fn parse(
  s: &str,
  add: for<'a> fn(&mut Graph<&'a str>, &'a str, &'a str),
) -> Graph<&str> {
  let mut ret = Graph::new();
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
