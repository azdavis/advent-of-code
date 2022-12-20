use helpers::{hash_map, HashMap, HashSet};
use std::hash::Hash;

pub fn p1(s: &str) -> usize {
  let inp = process(s);
  ore_for_fuel(&inp, 1)
}

pub fn p2(s: &str) -> usize {
  let inp = process(s);
  let one = ore_for_fuel(&inp, 1);
  let mut lo = MAX_ORE / one;
  let mut hi = MAX_ORE;
  while lo < hi - 1 {
    let mid = lo + ((hi - lo) / 2);
    if ore_for_fuel(&inp, mid) < MAX_ORE {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  lo
}

/// Returns `ceil(a / b)`.
pub fn ceil_div(a: usize, b: usize) -> usize {
  let mut ret = a / b;
  if a % b != 0 {
    ret += 1;
  }
  ret
}

#[test]
fn t_ceil_div() {
  assert_eq!(ceil_div(5, 1), 5);
  assert_eq!(ceil_div(2, 1), 2);
  assert_eq!(ceil_div(6, 2), 3);
  assert_eq!(ceil_div(5, 2), 3);
  assert_eq!(ceil_div(4, 2), 2);
  assert_eq!(ceil_div(5, 3), 2);
  assert_eq!(ceil_div(3, 3), 1);
  assert_eq!(ceil_div(2, 3), 1);
  assert_eq!(ceil_div(0, 3), 0);
}

const MAX_ORE: usize = 1_000_000_000_000;

fn ore_for_fuel(inp: &Input<'_>, num_fuel: usize) -> usize {
  let mut want = hash_map([("FUEL", num_fuel)]);
  for &chem in inp.order.iter() {
    let num_need = want.remove(chem).unwrap();
    let (per_batch, ref ins) = inp.recipes[&chem];
    let num_batches = ceil_div(num_need, per_batch);
    for ing in ins.iter() {
      *want.entry(ing.chem).or_default() += num_batches * ing.count;
    }
  }
  assert_eq!(want.len(), 1);
  want["ORE"]
}

struct Input<'a> {
  recipes: HashMap<&'a str, (usize, Vec<Elem<'a>>)>,
  order: Vec<&'a str>,
}

fn process(s: &str) -> Input<'_> {
  let recipes: HashMap<_, _> = s
    .lines()
    .map(|line| {
      let mut parts = line.split(" => ");
      let ins: Vec<_> = parts.next().unwrap().split(", ").map(parse_elem).collect();
      let out = parse_elem(parts.next().unwrap());
      assert!(parts.next().is_none());
      (out.chem, (out.count, ins))
    })
    .collect();
  let mut graph = Graph::default();
  for (&out, &(_, ref ins)) in recipes.iter() {
    for ing in ins.iter() {
      graph.entry(ing.chem).or_default().insert(out);
    }
  }
  let order = topological_sort("ORE", &graph);
  Input { recipes, order }
}

type Graph<T> = HashMap<T, HashSet<T>>;

enum Action {
  Start,
  Finish,
}

/// topological sort via DFS. returns a reverse topological ordering of the
/// subgraph of `graph` reachable from `start`, not including `start` (it would
/// be last). panics if this is not a DAG.
fn topological_sort<T>(start: T, graph: &Graph<T>) -> Vec<T>
where
  T: Hash + Eq + Copy,
{
  let mut active = HashSet::default();
  let mut done = HashSet::default();
  let mut order = Vec::new();
  let mut stack = vec![(Action::Start, start)];
  while let Some((ac, cur)) = stack.pop() {
    match ac {
      Action::Start => {
        if done.contains(&cur) {
          continue;
        }
        if !active.insert(cur) {
          panic!("not a DAG");
        }
        stack.push((Action::Finish, cur));
        if let Some(ns) = graph.get(&cur) {
          stack.extend(ns.iter().map(|&x| (Action::Start, x)));
        }
      }
      Action::Finish => {
        assert!(active.remove(&cur));
        assert!(done.insert(cur));
        order.push(cur);
      }
    }
  }
  // don't require T: Debug
  assert!(order.pop().unwrap() == start, "last elem was not start");
  order
}

#[derive(Debug)]
struct Elem<'a> {
  count: usize,
  chem: &'a str,
}

fn parse_elem(s: &str) -> Elem<'_> {
  let mut parts = s.split(' ');
  let count: usize = parts.next().unwrap().parse().unwrap();
  let chem = parts.next().unwrap();
  assert!(parts.next().is_none());
  Elem { count, chem }
}

#[test]
fn t_p1() {
  let s = include_str!("input/d14_ex1.txt");
  assert_eq!(p1(s), 31);
  let s = include_str!("input/d14_ex2.txt");
  assert_eq!(p1(s), 165);
  let s = include_str!("input/d14_ex3.txt");
  assert_eq!(p1(s), 13312);
  let s = include_str!("input/d14_ex4.txt");
  assert_eq!(p1(s), 180697);
  let s = include_str!("input/d14_ex5.txt");
  assert_eq!(p1(s), 2210736);
}

#[test]
fn t_p2() {
  let s = include_str!("input/d14_ex3.txt");
  assert_eq!(p2(s), 82892753);
  let s = include_str!("input/d14_ex4.txt");
  assert_eq!(p2(s), 5586022);
  let s = include_str!("input/d14_ex5.txt");
  assert_eq!(p2(s), 460664);
}

#[test]
fn t() {
  let s = include_str!("input/d14.txt");
  assert_eq!(p1(s), 870051);
  assert_eq!(p2(s), 1863741);
}
