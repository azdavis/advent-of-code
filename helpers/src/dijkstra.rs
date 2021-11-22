//! Dijkstra's algorithm for graphs.

use crate::infinitable::Infinitable;
use crate::{HashMap, HashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

/// The graph trait. TODO the `'a` is a hack around the absence of GATs at time
/// of writing.
pub trait Graph<'a> {
  /// The type of nodes in this graph.
  type Node: 'a + Hash + Ord + Copy;

  /// The iterator returned by [`Self::nodes`].
  type Nodes: 'a + Iterator<Item = Self::Node>;

  /// Returns an iterator over all the nodes in the graph.
  fn nodes(&'a self) -> Self::Nodes;

  /// The iterator returned by [`Self::neighbors`].
  type Neighbors: 'a + Iterator<Item = Self::Node>;

  /// Returns an iterator over the neighbors of `node` in the graph.
  fn neighbors(&'a self, node: Self::Node) -> Self::Neighbors;
}

/// The algorithm. Don't need to store predecessors info (cf wikipedia).
pub fn dijkstra<'a, G>(
  graph: &'a G,
  start: G::Node,
  end: G::Node,
) -> Option<usize>
where
  G: Graph<'a>,
{
  let mut distances: HashMap<_, _> = graph
    .nodes()
    .map(|node| (node, Infinitable::PosInf))
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
    for node in graph.neighbors(u.node) {
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

/// A graph implemented with maps and sets.
#[derive(Debug)]
pub struct MapGraph<T>(HashMap<T, HashSet<T>>);

impl<T> MapGraph<T>
where
  T: Hash + Eq,
{
  /// Returns the set of neighbors for this node.
  pub fn get(&self, node: T) -> Option<&HashSet<T>> {
    self.0.get(&node)
  }

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

impl<'a, T> Graph<'a> for MapGraph<T>
where
  T: 'a + Hash + Ord + Copy,
{
  type Node = T;

  type Nodes =
    std::iter::Copied<std::collections::hash_map::Keys<'a, T, HashSet<T>>>;

  fn nodes(&'a self) -> Self::Nodes {
    self.0.keys().copied()
  }

  type Neighbors = std::iter::Copied<
    std::iter::Flatten<std::option::IntoIter<&'a HashSet<T>>>,
  >;

  fn neighbors(&'a self, node: Self::Node) -> Self::Neighbors {
    self.0.get(&node).into_iter().flatten().copied()
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
