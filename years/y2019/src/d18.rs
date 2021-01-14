use helpers::bit_set::BitSet;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn p1(s: &str) -> usize {
  let graph = parse(s);
  // depends on no dupe keys
  let num_keys = graph
    .keys()
    .filter(|&&k| MIN_KEY <= k && k <= MAX_KEY)
    .count() as u32;
  let mut states = vec![State {
    at: START,
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
        if MIN_KEY <= at && at <= MAX_KEY && !st.keys.contains(at) {
          let mut keys = st.keys;
          keys.insert(at);
          // steps(start -> at) = steps(start -> st.at) + steps(st.at -> at).
          let steps = st.steps + s_at;
          if cache.get(&(at, keys)).map_or(true, |&x| x > steps) {
            cache.insert((at, keys), steps);
            states.push(State { at, keys, steps });
          }
          continue;
        }
        if MIN_DOOR <= at
          && at <= MAX_DOOR
          && !st.keys.contains(at - MIN_DOOR + MIN_KEY)
        {
          continue;
        }
        queue.extend(graph[&at].iter().map(|&(s_n, n)| (s_n + s_at, n)));
      }
    }
  }
  min_steps.unwrap()
}

pub fn p2(_: &str) -> u32 {
  todo!()
}

type Node = u8;
const START: Node = 0;
const MIN_KEY: Node = 1;
const MAX_KEY: Node = 26;
const MIN_DOOR: Node = 27;
const MAX_DOOR: Node = 52;

type Graph = HashMap<Node, HashSet<(usize, Node)>>;

/// a tuple of (x, y).
type Point = (usize, usize);

#[derive(Debug)]
struct State {
  at: Node,
  keys: BitSet,
  steps: usize,
}

fn parse(s: &str) -> Graph {
  let mut walkable = HashSet::<Point>::new();
  let mut nodes = HashMap::<Point, Node>::new();
  for (y, line) in s.lines().enumerate() {
    for (x, b) in line.bytes().enumerate() {
      match b {
        b'@' => {
          nodes.insert((x, y), START);
        }
        b'.' => {}
        b'#' => continue,
        _ => {
          if b.is_ascii_lowercase() {
            let key = b - b'a' + MIN_KEY;
            assert!(MIN_KEY <= key && key <= MAX_KEY);
            nodes.insert((x, y), key);
          } else if b.is_ascii_uppercase() {
            let door = b - b'A' + MIN_DOOR;
            assert!(MIN_DOOR <= door && door <= MAX_DOOR);
            nodes.insert((x, y), door);
          } else {
            panic!("bad byte: {}", b)
          }
        }
      }
      walkable.insert((x, y));
    }
  }
  let mut ret = Graph::new();
  let mut visited = HashSet::<Point>::new();
  let mut queue = VecDeque::<Point>::new();
  for (&point, &node) in nodes.iter() {
    let mut steps = 0;
    visited.clear();
    queue.clear();
    queue.push_back(point);
    while !queue.is_empty() {
      for _ in 0..queue.len() {
        let point = queue.pop_front().unwrap();
        if !visited.insert(point) || !walkable.contains(&point) {
          continue;
        }
        if let Some(&n) = nodes.get(&point) {
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
