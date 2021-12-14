use helpers::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

const START: &str = "start";
const END: &str = "end";

fn num_paths<'a>(
  graph: &Graph<'a>,
  visited: &mut HashSet<&'a str>,
  re_visit: Option<&'a str>,
  cur: &'a str,
) -> usize {
  if cur == END {
    return match re_visit {
      None => 1,
      Some(x) => {
        if visited.contains(x) {
          1
        } else {
          // don't double-count, will be handled in the second case
          0
        }
      }
    };
  }
  if visited.contains(&cur) {
    return 0;
  }
  let mut ret = 0usize;
  if cur.chars().all(|c| c.is_ascii_lowercase()) {
    if cur != START && re_visit.is_none() {
      let re_visit = Some(cur);
      ret += graph[cur]
        .iter()
        .map(|&cur| num_paths(graph, &mut visited.clone(), re_visit, cur))
        .sum::<usize>();
    }
    visited.insert(cur);
  }
  ret += graph[cur]
    .iter()
    .map(|&cur| num_paths(graph, &mut visited.clone(), re_visit, cur))
    .sum::<usize>();
  ret
}

fn run(s: &str, re_visit: Option<&str>) -> usize {
  let mut graph = Graph::default();
  for line in s.lines() {
    let (a, b) = line.split_once('-').unwrap();
    graph.entry(a).or_default().insert(b);
    graph.entry(b).or_default().insert(a);
  }
  num_paths(&graph, &mut HashSet::default(), re_visit, START)
}

pub fn p1(s: &str) -> usize {
  // Some(START) is just a hack to make sure we don't re-visit anything
  run(s, Some(START))
}

pub fn p2(s: &str) -> usize {
  run(s, None)
}

#[test]
fn t() {
  let s = include_str!("input/d12.txt");
  assert_eq!(p1(s), 3887);
  assert_eq!(p2(s), 104834);
}

#[test]
fn ex1() {
  let s = include_str!("input/d12_ex1.txt");
  assert_eq!(p1(s), 10);
  assert_eq!(p2(s), 36);
}

#[test]
fn ex2() {
  let s = include_str!("input/d12_ex2.txt");
  assert_eq!(p1(s), 19);
  assert_eq!(p2(s), 103);
}
