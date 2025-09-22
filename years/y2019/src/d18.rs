mod bit_set;

use bit_set::BitSet;
use helpers::{HashMap, HashSet};
use std::collections::VecDeque;

pub fn p1(s: &str) -> usize {
  go(s, vec![Node::Start1], |input| {
    input.nodes.insert(input.start, Node::Start1);
  })
}

pub fn p2(s: &str) -> usize {
  go(
    s,
    vec![Node::Start1, Node::Start2, Node::Start3, Node::Start4],
    prepare_input_p2,
  )
}

fn prepare_input_p2(input: &mut Input) {
  let (x, y) = input.start;
  for point in [(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
    assert!(input.walkable.remove(&point));
  }
  for (point, node) in [
    ((x - 1, y - 1), Node::Start1),
    ((x - 1, y + 1), Node::Start2),
    ((x + 1, y - 1), Node::Start3),
    ((x + 1, y + 1), Node::Start4),
  ] {
    assert!(input.nodes.insert(point, node).is_none());
  }
}

#[derive(Debug)]
struct State {
  at: Vec<Node>,
  keys: BitSet,
  steps: usize,
}

fn go(s: &str, init: Vec<Node>, prepare_input: fn(&mut Input)) -> usize {
  let mut input = parse(s);
  prepare_input(&mut input);
  let graph = mk_graph(&input);
  // depends on no dupe keys
  let num_keys: u32 = graph
    .keys()
    .filter(|&&node| matches!(node, Node::Key(_)))
    .count()
    .try_into()
    .unwrap();
  let mut states = vec![State {
    at: init,
    keys: BitSet::default(),
    steps: 0,
  }];
  let mut cache = HashMap::<(Vec<Node>, BitSet), usize>::default();
  let mut queue = VecDeque::<(usize, Node)>::new();
  let mut visited = HashSet::<Node>::default();
  let mut min_steps: Option<usize> = None;
  while let Some(st) = states.pop() {
    if st.keys.len() == num_keys {
      min_steps = Some(min_steps.map_or(st.steps, |ac| ac.min(st.steps)));
      continue;
    }
    for (node_idx, &node) in st.at.iter().enumerate() {
      queue.clear();
      visited.clear();
      queue.push_back((0, node));
      while !queue.is_empty() {
        for _ in 0..queue.len() {
          let (steps, node) = queue.pop_front().unwrap();
          if !visited.insert(node) {
            continue;
          }
          match node {
            Node::Start1 | Node::Start2 | Node::Start3 | Node::Start4 => {}
            Node::Key(k) => {
              if !st.keys.contains(k) {
                let mut keys = st.keys;
                keys.insert(k);
                let steps = st.steps + steps;
                let mut at = st.at.clone();
                at[node_idx] = node;
                let tup = (at.clone(), keys);
                if cache.get(&tup).is_none_or(|&x| x > steps) {
                  cache.insert(tup, steps);
                  states.push(State { at, keys, steps });
                }
                continue;
              }
            }
            Node::Door(k) => {
              if !st.keys.contains(k) {
                continue;
              }
            }
          }
          queue.extend(
            graph[&node]
              .iter()
              .copied()
              .map(|(s_n, n)| (steps + s_n, n)),
          );
        }
      }
    }
  }
  min_steps.unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
  Start1,
  Start2,
  Start3,
  Start4,
  Key(u8),
  Door(u8),
}

type Graph = HashMap<Node, HashSet<(usize, Node)>>;

/// a tuple of (x, y).
type Point = (usize, usize);

#[derive(Debug, Default)]
struct Input {
  walkable: HashSet<Point>,
  nodes: HashMap<Point, Node>,
  start: Point,
}

/// the returned `nodes` contains only Key and Door nodes
fn parse(s: &str) -> Input {
  let mut ret = Input::default();
  for (y, line) in s.lines().enumerate() {
    for (x, b) in line.bytes().enumerate() {
      match b {
        b'@' => ret.start = (x, y),
        b'.' => {}
        b'#' => continue,
        _ => {
          if b.is_ascii_lowercase() {
            ret.nodes.insert((x, y), Node::Key(b - b'a'));
          } else if b.is_ascii_uppercase() {
            ret.nodes.insert((x, y), Node::Door(b - b'A'));
          } else {
            panic!("bad byte: {b}")
          }
        }
      }
      ret.walkable.insert((x, y));
    }
  }
  ret
}

fn mk_graph(input: &Input) -> Graph {
  let mut ret = Graph::default();
  let mut visited = HashSet::<Point>::default();
  let mut queue = VecDeque::<Point>::new();
  for (&point, &node) in &input.nodes {
    let mut steps = 0;
    visited.clear();
    queue.clear();
    queue.push_back(point);
    while !queue.is_empty() {
      for _ in 0..queue.len() {
        let point = queue.pop_front().unwrap();
        if !visited.insert(point) || !input.walkable.contains(&point) {
          continue;
        }
        if let Some(&n) = input.nodes.get(&point) {
          if node != n {
            ret.entry(node).or_default().insert((steps, n));
            continue;
          }
        }
        let (x, y) = point;
        let neighbors = [
          x.checked_sub(1).map(|x| (x, y)),
          x.checked_add(1).map(|x| (x, y)),
          y.checked_sub(1).map(|y| (x, y)),
          y.checked_add(1).map(|y| (x, y)),
        ];
        queue.extend(neighbors.iter().flatten().copied());
      }
      steps += 1;
    }
  }
  ret
}

#[test]
fn t() {
  let s = include_str!("input/d18.txt");
  assert_eq!(p1(s), 5068);
  assert_eq!(p2(s), 1966);
}

#[test]
fn t_p1() {
  let s = include_str!("input/d18_ex1.txt");
  assert_eq!(p1(s), 8);
  let s = include_str!("input/d18_ex2.txt");
  assert_eq!(p1(s), 86);
}

#[test]
fn t_p2() {
  let s = include_str!("input/d18_ex3.txt");
  assert_eq!(p2(s), 8);
}
