//! Dijkstra's algorithm for graphs.

use crate::infinitable::Infinitable;
use crate::{HashMap, HashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

/// The graph trait.
///
/// NOTE the methods would return iterators if they were better supported
/// (generators, GATs, nameable closure types, etc)
pub trait Graph {
  /// The type of nodes in this graph.
  type Node;

  /// Returns the neighbors of `node` in the graph.
  fn neighbors(&self, node: Self::Node) -> HashSet<Self::Node>;

  /// Returns the distance between the nodes in the graph.
  ///
  /// The default implementation always returns 1.
  fn distance(&self, _: Self::Node, _: Self::Node) -> usize {
    1
  }
}

/// Returns the length of the shortest path from `start` to `end` in `graph`, or
/// `None` if there is no such path.
pub fn dijkstra<G>(graph: &G, start: G::Node, end: G::Node) -> Option<usize>
where
  G: Graph,
  G::Node: Hash + Ord + Copy,
{
  // Don't need to store predecessors info (cf wikipedia).
  let mut distances = HashMap::default();
  distances.insert(start, Infinitable::Finite(0));
  let mut pq: BinaryHeap<_> = distances
    .iter()
    .map(|(&node, &dist)| Elem::new(node, dist))
    .collect();
  while let Some(u) = pq.pop() {
    let u_dist = distances
      .get(&u.node)
      .copied()
      .unwrap_or(Infinitable::PosInf);
    if u.node == end {
      match u_dist {
        Infinitable::Finite(x) => return Some(x),
        _ => unreachable!(),
      }
    }
    if u.dist.0 > u_dist {
      continue;
    }
    for node in graph.neighbors(u.node) {
      let old_dist =
        distances.get(&node).copied().unwrap_or(Infinitable::PosInf);
      let new_dist = u_dist + graph.distance(u.node, node);
      if new_dist >= old_dist {
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

/// A graph implemented with maps and sets.
#[derive(Debug)]
pub struct MapGraph<T>(HashMap<T, HashSet<T>>);

impl<T> MapGraph<T>
where
  T: Hash + Eq,
{
  /// Returns an entry for this node.
  pub fn entry(
    &mut self,
    node: T,
  ) -> std::collections::hash_map::Entry<'_, T, HashSet<T>> {
    self.0.entry(node)
  }
}

impl<T> Default for MapGraph<T> {
  fn default() -> Self {
    Self(HashMap::default())
  }
}

impl<T> Graph for MapGraph<T>
where
  T: Hash + Eq + Copy,
{
  type Node = T;

  fn neighbors(&self, node: Self::Node) -> HashSet<Self::Node> {
    self.0.get(&node).into_iter().flatten().copied().collect()
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
