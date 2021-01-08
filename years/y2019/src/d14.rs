use helpers::{ceil_div::ceil_div, maplit::hashmap};
use std::collections::{HashMap, HashSet};
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

const MAX_ORE: usize = 1_000_000_000_000;

fn ore_for_fuel(inp: &Input<'_>, num_fuel: usize) -> usize {
  let mut want = hashmap!["FUEL" => num_fuel];
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

fn process(s: &str) -> Input {
  let recipes: HashMap<_, _> = s
    .lines()
    .map(|line| {
      let mut parts = line.split(" => ");
      let ins: Vec<_> =
        parts.next().unwrap().split(", ").map(parse_elem).collect();
      let out = parse_elem(parts.next().unwrap());
      assert!(parts.next().is_none());
      (out.chem, (out.count, ins))
    })
    .collect();
  let mut graph = Graph::new();
  for (&out, &(_, ref ins)) in recipes.iter() {
    for ing in ins.iter() {
      graph.entry(ing.chem).or_default().insert(out);
    }
  }
  let order = topological_sort("ORE", &graph);
  Input { recipes, order }
}

type Graph<T> = HashMap<T, HashSet<T>>;

/// topological sort via DFS. returns a reverse topological ordering of the
/// subgraph of `graph` reachable from `start`, not including `start` (it would
/// be last). panics if this is not a DAG.
fn topological_sort<T>(start: T, graph: &Graph<T>) -> Vec<T>
where
  T: Hash + Eq + Copy,
{
  let mut st = State::default();
  topological_sort_go(start, graph, &mut st);
  // don't require T: Debug
  assert!(st.order.pop().unwrap() == start, "last elem was not start");
  st.order
}

struct State<T> {
  active: HashSet<T>,
  done: HashSet<T>,
  order: Vec<T>,
}

impl<T> Default for State<T> {
  fn default() -> Self {
    Self {
      active: HashSet::default(),
      done: HashSet::default(),
      order: Vec::default(),
    }
  }
}

fn topological_sort_go<T>(cur: T, graph: &Graph<T>, st: &mut State<T>)
where
  T: Hash + Eq + Copy,
{
  if st.done.contains(&cur) {
    return;
  }
  if !st.active.insert(cur) {
    panic!("not a DAG");
  }
  // use `Option#into_iter` and `Iterator#flatten` to skip the loop if `get`
  // returns `None`.
  for &x in graph.get(&cur).into_iter().flatten() {
    topological_sort_go(x, graph, st);
  }
  assert!(st.active.remove(&cur));
  assert!(st.done.insert(cur));
  st.order.push(cur);
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
