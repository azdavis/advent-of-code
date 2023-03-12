use crate::intern::Intern;
use helpers::HashMap;
use helpers::{permute, static_regex};

static_regex!(RE = r"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).$");

type Graph = HashMap<[usize; 2], i16>;

fn parse(s: &str) -> (Graph, usize) {
  let mut intern = Intern::default();
  let graph: Graph = s
    .lines()
    .map(|line| {
      let caps = RE.captures(line).unwrap();
      let a = intern.get(caps.get(1).unwrap().as_str());
      let sign: i16 = match &caps[2] {
        "gain" => 1,
        "lose" => -1,
        s => panic!("unknown sign: {s}"),
      };
      let points: i16 = caps[3].parse().unwrap();
      let b = intern.get(caps.get(4).unwrap().as_str());
      ([a, b], sign * points)
    })
    .collect();
  (graph, intern.len())
}

fn happiness(graph: &Graph, a: usize, b: usize) -> i16 {
  graph[&[a, b]] + graph[&[b, a]]
}

fn run(graph: &Graph, len: usize) -> i16 {
  permute(0..len)
    .into_iter()
    .map(|perm| {
      let &first = perm.first().unwrap();
      let &last = perm.last().unwrap();
      perm
        .windows(2)
        .map(|w| happiness(graph, w[0], w[1]))
        .chain(std::iter::once(happiness(graph, first, last)))
        .sum::<i16>()
    })
    .max()
    .unwrap()
}

pub fn p1(s: &str) -> i16 {
  let (graph, len) = parse(s);
  run(&graph, len)
}

pub fn p2(s: &str) -> i16 {
  let (mut graph, len) = parse(s);
  let me = len;
  for other in 0..len {
    graph.insert([me, other], 0);
    graph.insert([other, me], 0);
  }
  run(&graph, len + 1)
}

#[test]
fn t() {
  let s = include_str!("input/d13.txt");
  assert_eq!(p1(s), 733);
  assert_eq!(p2(s), 725);
}

#[test]
fn ex1() {
  let s = include_str!("input/d13_ex1.txt");
  assert_eq!(p1(s), 330);
}
