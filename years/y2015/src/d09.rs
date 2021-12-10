use helpers::{hash_set, HashMap, HashSet};

type Node = usize;
type Edge = [Node; 2];
type Dist = u16;
type Edges = HashMap<Edge, Dist>;

#[derive(Default)]
struct NodeIntern<'a> {
  next: Node,
  map: HashMap<&'a str, Node>,
}

impl<'a> NodeIntern<'a> {
  fn get(&mut self, s: &'a str) -> Node {
    if let Some(&n) = self.map.get(s) {
      return n;
    }
    let ret = self.next;
    self.map.insert(s, ret);
    self.next += 1;
    ret
  }
}

fn edge(a: Node, b: Node) -> Edge {
  if a <= b {
    [a, b]
  } else {
    [b, a]
  }
}

fn parse(s: &str) -> (Edges, Node) {
  let mut node_intern = NodeIntern::default();
  let mut edges = Edges::default();
  for line in s.lines() {
    let mut iter = line.split_ascii_whitespace();
    let a = node_intern.get(iter.next().unwrap());
    assert_eq!(iter.next().unwrap(), "to");
    let b = node_intern.get(iter.next().unwrap());
    assert_eq!(iter.next().unwrap(), "=");
    let dist: Dist = iter.next().unwrap().parse().unwrap();
    assert!(iter.next().is_none());
    edges.insert(edge(a, b), dist);
  }
  (edges, node_intern.next)
}

fn help(
  op: fn(Dist, Dist) -> Dist,
  max: Node,
  edges: &Edges,
  visited: &mut HashSet<Node>,
  at: Node,
) -> Dist {
  (0..max)
    .filter_map(|node| {
      if visited.contains(&node) {
        return None;
      }
      visited.insert(node);
      let ret = edges[&edge(at, node)] + help(op, max, edges, visited, node);
      visited.remove(&node);
      Some(ret)
    })
    .reduce(op)
    .unwrap_or(0)
}

fn run(s: &str, op: fn(Dist, Dist) -> Dist) -> Dist {
  let (edges, max) = parse(s);
  (0..max)
    .map(|node| help(op, max, &edges, &mut hash_set([node]), node))
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
