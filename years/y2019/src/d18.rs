use helpers::bit_set::BitSet;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn p1(s: &str) -> usize {
  go(s)
}

pub fn p2(_: &str) -> u32 {
  todo!()
}

fn go(s: &str) -> usize {
  let graph = mk_graph(parse(s));
  // depends on no dupe keys
  let num_keys = graph
    .keys()
    .filter(|&&node| matches!(node, Node::Key(_)))
    .count() as u32;
  let mut states = vec![State {
    at: Node::Start,
    keys: BitSet::new(),
    steps: 0,
  }];
  let mut cache = HashMap::<(Node, BitSet), usize>::new();
  let mut queue = VecDeque::<(usize, Node)>::new();
  let mut visited = HashSet::<Node>::new();
  let mut min_steps: Option<usize> = None;
  while let Some(st) = states.pop() {
    if st.keys.len() == num_keys {
      min_steps = Some(min_steps.map_or(st.steps, |ac| ac.min(st.steps)));
      continue;
    }
    queue.clear();
    visited.clear();
    queue.push_back((0, st.at));
    while !queue.is_empty() {
      for _ in 0..queue.len() {
        let (s_at, at) = queue.pop_front().unwrap();
        if !visited.insert(at) {
          continue;
        }
        match at {
          Node::Start => {}
          Node::Key(k) => {
            if !st.keys.contains(k) {
              let mut keys = st.keys;
              keys.insert(k);
              // steps(start, at) = steps(start, st.at) + steps(st.at, at).
              let steps = st.steps + s_at;
              if cache.get(&(at, keys)).map_or(true, |&x| x > steps) {
                cache.insert((at, keys), steps);
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
        queue.extend(graph[&at].iter().map(|&(s_n, n)| (s_n + s_at, n)));
      }
    }
  }
  min_steps.unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
  Start,
  Key(u8),
  Door(u8),
}

type Graph = HashMap<Node, HashSet<(usize, Node)>>;

/// a tuple of (x, y).
type Point = (usize, usize);

#[derive(Debug)]
struct State<T> {
  at: T,
  keys: BitSet,
  steps: usize,
}

#[derive(Debug, Default)]
struct Input {
  walkable: HashSet<Point>,
  nodes: HashMap<Point, Node>,
}

fn parse(s: &str) -> Input {
  let mut ret = Input::default();
  for (y, line) in s.lines().enumerate() {
    for (x, b) in line.bytes().enumerate() {
      match b {
        b'@' => {
          ret.nodes.insert((x, y), Node::Start);
        }
        b'.' => {}
        b'#' => continue,
        _ => {
          if b.is_ascii_lowercase() {
            ret.nodes.insert((x, y), Node::Key(b - b'a'));
          } else if b.is_ascii_uppercase() {
            ret.nodes.insert((x, y), Node::Door(b - b'A'));
          } else {
            panic!("bad byte: {}", b)
          }
        }
      }
      ret.walkable.insert((x, y));
    }
  }
  ret
}

fn mk_graph(input: Input) -> Graph {
  let mut ret = Graph::new();
  let mut visited = HashSet::<Point>::new();
  let mut queue = VecDeque::<Point>::new();
  for (&point, &node) in input.nodes.iter() {
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
  // assert_eq!(p2(s), ___);
}

#[test]
fn t_p1() {
  let s = include_str!("input/d18_ex1.txt");
  assert_eq!(p1(s), 8);
  let s = include_str!("input/d18_ex2.txt");
  assert_eq!(p1(s), 86);
}
