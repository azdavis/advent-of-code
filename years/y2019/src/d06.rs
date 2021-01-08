use helpers::infinitable::Infinitable;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub fn p1(s: &str) -> usize {
  let inp = parse(s, |inp, center, orbiter| {
    inp.entry(center).or_default().insert(orbiter);
  });
  let mut visited = HashSet::new();
  let mut queue = VecDeque::from(vec!["COM"]);
  let mut level = 0;
  let mut ret = 0;
  loop {
    let level_len = queue.len();
    if level_len == 0 {
      break;
    }
    for _ in 0..level_len {
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
    .map(|&val| (val, Infinitable::PosInf))
    .collect();
  distances.insert(start, Infinitable::Finite(0));
  let mut pq: BinaryHeap<_> = distances
    .iter()
    .map(|(&val, &v)| Elem {
      val,
      dist: Reverse(v),
    })
    .collect();
  while let Some(u) = pq.pop() {
    let d_u = *distances.get(&u.val).unwrap();
    if u.val == end {
      match d_u {
        Infinitable::Finite(x) => return Some(x),
        _ => unreachable!(),
      }
    }
    if u.dist.0 > d_u {
      continue;
    }
    let alt = d_u + 1;
    for &v in graph.get(&u.val).into_iter().flatten() {
      if alt >= *distances.get(&v).unwrap() {
        continue;
      }
      distances.insert(v, alt);
      pq.push(Elem {
        val: v,
        dist: Reverse(alt),
      });
    }
  }
  None
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Elem<T> {
  dist: Reverse<Infinitable<usize>>,
  val: T,
}

#[test]
fn elem_cmp() {
  let a = Elem {
    dist: Reverse(Infinitable::PosInf),
    val: "a",
  };
  let b = Elem {
    dist: Reverse(Infinitable::Finite(3)),
    val: "a",
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
