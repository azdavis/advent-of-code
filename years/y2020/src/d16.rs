use helpers::{HashMap, HashSet};
use std::ops::RangeInclusive;

pub fn p1(s: &str) -> u64 {
  let (fields, _, nearby) = parse(s);
  nearby
    .iter()
    .flatten()
    .filter(|&&num| fields.values().all(|ranges| !ranges.contains(num)))
    .sum()
}

pub fn p2(s: &str) -> u64 {
  let (fields, me, mut nearby) = parse(s);
  nearby.retain(|ticket| {
    ticket
      .iter()
      .all(|&num| fields.values().any(|ranges| ranges.contains(num)))
  });
  let mut candidates: Vec<_> = fields
    .iter()
    .map(|(&key, ranges)| {
      let set: HashSet<_> = (0..fields.len())
        .filter(|&idx| nearby.iter().all(|ticket| ranges.contains(ticket[idx])))
        .collect();
      (key, set)
    })
    .collect();
  candidates.sort_unstable_by_key(|&(key, ref set)| (set.len(), key));
  let mut assigned = HashSet::default();
  let mut mapping = HashMap::default();
  for &(key, ref set) in &candidates {
    assert_eq!(assigned.len(), mapping.len());
    assert_eq!(assigned.len() + 1, set.len());
    let mut iter = set.difference(&assigned);
    let got = *iter.next().unwrap();
    assert!(iter.next().is_none());
    assigned.insert(got);
    mapping.insert(key, got);
  }
  mapping
    .into_iter()
    .filter(|&(key, _)| key.starts_with("departure"))
    .map(|(_, idx)| me[idx])
    .product()
}

type Ticket = Vec<u64>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Ranges {
  fst: RangeInclusive<u64>,
  snd: RangeInclusive<u64>,
}

impl Ranges {
  fn contains(&self, num: u64) -> bool {
    self.fst.contains(&num) || self.snd.contains(&num)
  }
}

fn parse(s: &str) -> (HashMap<&str, Ranges>, Ticket, Vec<Ticket>) {
  let mut lines = s.lines();
  let mut fields = HashMap::default();
  loop {
    let line = lines.next().unwrap();
    if line.is_empty() {
      break;
    }
    let mut parts = line.split(": ");
    let key = parts.next().unwrap();
    let mut parts = parts.next().unwrap().split(" or ");
    let fst = parse_range(parts.next().unwrap());
    let snd = parse_range(parts.next().unwrap());
    fields.insert(key, Ranges { fst, snd });
  }
  assert_eq!(lines.next().unwrap(), "your ticket:");
  let me = parse_ticket(lines.next().unwrap());
  assert!(lines.next().unwrap().is_empty());
  assert_eq!(lines.next().unwrap(), "nearby tickets:");
  let nearby: Vec<_> = lines.map(parse_ticket).collect();
  for n in &nearby {
    assert_eq!(fields.len(), n.len());
  }
  assert_eq!(fields.len(), me.len());
  (fields, me, nearby)
}

fn parse_ticket(s: &str) -> Ticket {
  s.split(',').map(|x| x.parse().unwrap()).collect()
}

fn parse_range(s: &str) -> RangeInclusive<u64> {
  let mut parts = s.split('-');
  let fst: u64 = parts.next().unwrap().parse().unwrap();
  let snd: u64 = parts.next().unwrap().parse().unwrap();
  fst..=snd
}

#[test]
fn t() {
  let s = include_str!("input/d16.txt");
  assert_eq!(p1(s), 25984);
  assert_eq!(p2(s), 1_265_347_500_049);
}
