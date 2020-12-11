use std::collections::{HashMap, HashSet};

fn mk_graph<'a, F, T>(s: &'a str, add: F) -> HashMap<Bag<'a>, HashSet<T>>
where
  F: Fn(Bag<'a>, u32, Bag<'a>) -> (Bag<'a>, T),
  T: std::hash::Hash + Eq + 'a,
{
  let mut map = HashMap::new();
  for line in s.split('\n') {
    if line.is_empty() {
      continue;
    }
    let mut iter = line.split_ascii_whitespace();
    let this = Bag {
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
      let num: u32 = match next.parse() {
        Ok(x) => x,
        Err(e) => panic!("error parsing {}: {}", next, e),
      };
      let other = Bag {
        adj: iter.next().unwrap(),
        color: iter.next().unwrap(),
      };
      let (key, val) = add(this, num, other);
      map.entry(key).or_insert_with(HashSet::new).insert(val);
      match iter.next().unwrap() {
        "bag," | "bags," => {}
        "bag." | "bags." => break,
        bad => panic!("bad next: {}", bad),
      }
      next = iter.next().unwrap();
    }
    assert!(iter.next().is_none());
  }
  map
}

pub fn p1(s: &str) -> usize {
  let map = mk_graph(s, |a, _, b| (b, a));
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
    let neighbors = match map.get(&bag) {
      None => continue,
      Some(x) => x,
    };
    for &n in neighbors {
      cur.push(n);
    }
  }
  visited.len() - 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Bag<'a> {
  adj: &'a str,
  color: &'a str,
}
