use helpers::{Counter, HashMap};

fn parse(s: &str) -> (&[u8], HashMap<[u8; 2], u8>) {
  let mut lines = s.lines();
  let polymer = lines.next().unwrap().as_bytes();
  assert!(lines.next().unwrap().is_empty());
  let rules: HashMap<[u8; 2], u8> = lines
    .map(|line| {
      let (a, b) = line.split_once(" -> ").unwrap();
      (a.as_bytes().try_into().unwrap(), b.bytes().next().unwrap())
    })
    .collect();
  (polymer, rules)
}

fn run(s: &str, rounds: usize) -> usize {
  let (polymer, rules) = parse(s);
  let mut active = Counter::<[u8; 2]>::default();
  // NOTE could use array_windows here.
  for w in polymer.windows(2) {
    let w: [u8; 2] = w.try_into().unwrap();
    active.inc(w);
  }
  for _ in 0..rounds {
    let mut new_active = Counter::<[u8; 2]>::default();
    for (pair, count) in active {
      let [a, c] = pair;
      let b = rules[&pair];
      new_active.add([a, b], count);
      new_active.add([b, c], count);
    }
    active = new_active;
  }
  let mut counts = Counter::<u8>::default();
  for ([a, _], count) in active {
    counts.add(a, count);
    // skip the other char to not double-count
  }
  // fix last char
  let &last = polymer.last().unwrap();
  counts.inc(last);
  let min = counts.iter().map(|(_, c)| c).min().unwrap();
  let max = counts.iter().map(|(_, c)| c).max().unwrap();
  max - min
}

/// works for p1 but not p2
#[allow(dead_code)]
fn run_naive(s: &str, rounds: usize) -> usize {
  let (polymer, rules) = parse(s);
  let mut polymer = polymer.to_owned();
  for _ in 0..rounds {
    let mut new_polymer = Vec::with_capacity(polymer.len() * 2);
    for w in polymer.windows(2) {
      new_polymer.push(w[0]);
      let &x = rules.get(w).unwrap();
      new_polymer.push(x);
    }
    new_polymer.push(polymer.pop().unwrap());
    polymer = new_polymer;
  }
  let mut counts = Counter::<u8>::default();
  for b in polymer {
    counts.inc(b);
  }
  let min = counts.iter().map(|(_, c)| c).min().unwrap();
  let max = counts.iter().map(|(_, c)| c).max().unwrap();
  max - min
}

pub fn p1(s: &str) -> usize {
  run(s, 10)
}

pub fn p2(s: &str) -> usize {
  run(s, 40)
}

#[test]
fn t() {
  let s = include_str!("input/d14.txt");
  assert_eq!(p1(s), 3831);
  assert_eq!(p2(s), 5725739914282);
}

#[test]
fn ex1() {
  let s = include_str!("input/d14_ex1.txt");
  assert_eq!(p1(s), 1588);
  assert_eq!(p2(s), 2188189693529);
}
