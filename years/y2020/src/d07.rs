use std::collections::{HashMap, HashSet};

pub fn p1(s: &str) -> usize {
  let graph = mk_graph(s, |a, _, b| (b, a));
  let start = Bag {
    adj: "shiny",
    color: "gold",
  };
  let mut visited = HashSet::new();
  let mut cur = vec![start];
  while let Some(bag) = cur.pop() {
    if !visited.insert(bag) {
      continue;
    }
    let neighbors = match graph.get(&bag) {
      None => continue,
      Some(x) => x,
    };
    for &n in neighbors {
      cur.push(n);
    }
  }
  visited.len() - 1
}

pub fn p2(s: &str) -> usize {
  let graph = mk_graph(s, |a, num, bag| (a, Edge { num, bag }));
  let start = Bag {
    adj: "shiny",
    color: "gold",
  };
  rec(start, &graph) - 1
}

fn rec<'a, 'b>(
  cur: Bag<'a>,
  graph: &HashMap<Bag<'b>, HashSet<Edge<'a>>>,
) -> usize {
  let neighbors = match graph.get(&cur) {
    None => return 1,
    Some(xs) => xs,
  };
  neighbors
    .iter()
    .fold(1, |ac, edge| ac + (edge.num * rec(edge.bag, graph)))
}

fn mk_graph<'a, F, T>(s: &'a str, add: F) -> HashMap<Bag<'a>, HashSet<T>>
where
  F: Fn(Bag<'a>, usize, Bag<'a>) -> (Bag<'a>, T),
  T: std::hash::Hash + Eq + 'a,
{
  let mut ret = HashMap::new();
  for line in s.split('\n') {
    if line.is_empty() {
      continue;
    }
    let mut iter = line.split_ascii_whitespace();
    let container = Bag {
      adj: iter.next().unwrap(),
      color: iter.next().unwrap(),
    };
    assert_eq!("bags", iter.next().unwrap());
    assert_eq!("contain", iter.next().unwrap());
    let mut next = iter.next().unwrap();
    if next == "no" {
      assert_eq!("other", iter.next().unwrap());
      assert_eq!("bags.", iter.next().unwrap());
      assert!(iter.next().is_none());
      continue;
    }
    loop {
      let num: usize = match next.parse() {
        Ok(x) => x,
        Err(e) => panic!("error parsing {}: {}", next, e),
      };
      let contained = Bag {
        adj: iter.next().unwrap(),
        color: iter.next().unwrap(),
      };
      let (key, val) = add(container, num, contained);
      ret.entry(key).or_insert_with(HashSet::new).insert(val);
      match iter.next().unwrap() {
        "bag," | "bags," => {}
        "bag." | "bags." => break,
        bad => panic!("bad next: {}", bad),
      }
      next = iter.next().unwrap();
    }
    assert!(iter.next().is_none());
  }
  ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Bag<'a> {
  adj: &'a str,
  color: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge<'a> {
  num: usize,
  bag: Bag<'a>,
}
