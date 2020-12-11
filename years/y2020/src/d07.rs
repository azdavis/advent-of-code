use std::collections::{HashMap, HashSet};

pub fn p1(s: &str) -> usize {
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
      let _: u32 = match next.parse() {
        Ok(x) => x,
        Err(e) => panic!("error parsing {}: {}", next, e),
      };
      let other = Bag {
        adj: iter.next().unwrap(),
        color: iter.next().unwrap(),
      };
      map.entry(other).or_insert_with(HashSet::new).insert(this);
      match iter.next().unwrap() {
        "bag," | "bags," => {}
        "bag." | "bags." => break,
        bad => panic!("bad next: {}", bad),
      }
      next = iter.next().unwrap();
    }
    assert!(iter.next().is_none());
  }
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
