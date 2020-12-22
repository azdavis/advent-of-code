use helpers::infinite::Infinite;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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
  dijkstra(inp, "YOU", "SAN").unwrap() - 2
}

fn dijkstra(inp: Input<'_>, start: &str, end: &str) -> Option<usize> {
  let mut distances: HashMap<_, _> =
    inp.keys().map(|&val| (val, Infinite::PosInf)).collect();
  distances.insert(start, Infinite::Finite(0));
  let mut predecessors = HashMap::new();
  let mut pq: BinaryHeap<Elem<'_>> = distances
    .iter()
    .map(|(&val, &v)| Elem {
      val,
      dist: Reverse(v),
    })
    .collect();
  while let Some(u) = pq.pop() {
    let d_u = *distances.get(u.val).unwrap();
    if u.val == end {
      match d_u {
        Infinite::Finite(x) => return Some(x),
        _ => unreachable!(),
      }
    }
    if u.dist.0 > d_u {
      continue;
    }
    let alt = d_u + 1;
    for &v in inp.get(u.val).into_iter().flatten() {
      if alt >= *distances.get(v).unwrap() {
        continue;
      }
      distances.insert(v, alt);
      predecessors.insert(v, u.val);
      pq.push(Elem {
        val: v,
        dist: Reverse(alt),
      });
    }
  }
  None
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Elem<'a> {
  dist: Reverse<Infinite<usize>>,
  val: &'a str,
}

#[test]
fn elem_cmp() {
  let a = Elem {
    dist: Reverse(Infinite::PosInf),
    val: "a",
  };
  let b = Elem {
    dist: Reverse(Infinite::Finite(3)),
    val: "a",
  };
  assert!(a < b);
}

type Input<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse<F>(s: &str, add: F) -> Input<'_>
where
  F: for<'a> Fn(&mut Input<'a>, &'a str, &'a str),
{
  let mut ret = Input::new();
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
  let inp = include_str!("input/d06.txt");
  assert_eq!(p1(inp), 150150);
  assert_eq!(p2(inp), 352);
}
