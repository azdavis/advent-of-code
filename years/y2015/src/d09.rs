use crate::intern::Intern;
use helpers::{hash_set, HashMap, HashSet};

type Node = usize;
type Edge = [Node; 2];
type Dist = u16;
type Graph = HashMap<Edge, Dist>;

fn edge(a: Node, b: Node) -> Edge {
  if a <= b {
    [a, b]
  } else {
    [b, a]
  }
}

fn parse(s: &str) -> (Graph, Node) {
  let mut intern = Intern::default();
  let mut graph = Graph::default();
  for line in s.lines() {
    let mut iter = line.split_ascii_whitespace();
    let a = intern.get(iter.next().unwrap());
    assert_eq!(iter.next().unwrap(), "to");
    let b = intern.get(iter.next().unwrap());
    assert_eq!(iter.next().unwrap(), "=");
    let dist: Dist = iter.next().unwrap().parse().unwrap();
    assert!(iter.next().is_none());
    graph.insert(edge(a, b), dist);
  }
  (graph, intern.len())
}

fn help(
  op: fn(Dist, Dist) -> Dist,
  max: Node,
  graph: &Graph,
  visited: &mut HashSet<Node>,
  at: Node,
) -> Dist {
  (0..max)
    .filter_map(|node| {
      if visited.contains(&node) {
        return None;
      }
      visited.insert(node);
      let ret = graph[&edge(at, node)] + help(op, max, graph, visited, node);
      visited.remove(&node);
      Some(ret)
    })
    .reduce(op)
    .unwrap_or(0)
}

fn run(s: &str, op: fn(Dist, Dist) -> Dist) -> Dist {
  let (graph, max) = parse(s);
  (0..max)
    .map(|node| help(op, max, &graph, &mut hash_set([node]), node))
    .reduce(op)
    .unwrap()
}

pub fn p1(s: &str) -> Dist {
  run(s, Dist::min)
}

pub fn p2(s: &str) -> Dist {
  run(s, Dist::max)
}

#[test]
fn t() {
  let s = include_str!("input/d09.txt");
  assert_eq!(p1(s), 141);
  assert_eq!(p2(s), 736);
}
