//! Dijkstra's algorithm for graphs.

use crate::infinitable::Infinitable;
use crate::{HashMap, HashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

/// The graph type.
pub type Graph<T> = HashMap<T, HashSet<T>>;

/// The algorithm. Don't need to store predecessors info (cf wikipedia).
pub fn dijkstra<T>(graph: &Graph<T>, start: T, end: T) -> Option<usize>
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
