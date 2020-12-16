use std::collections::HashMap;
use std::ops::RangeInclusive;

pub fn p1(s: &str) -> u32 {
  let (fields, _, nearby) = parse(s);
  nearby
    .iter()
    .flatten()
    .filter(|&num| fields.values().all(|ranges| !ranges.contains(num)))
    .sum()
}

pub fn p2(s: &str) -> u32 {
  todo!()
}

type Ticket = Vec<u32>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Ranges {
  fst: RangeInclusive<u32>,
  snd: RangeInclusive<u32>,
}

impl Ranges {
  fn contains(&self, num: &u32) -> bool {
    self.fst.contains(num) || self.snd.contains(num)
  }
}

fn parse(s: &str) -> (HashMap<&str, Ranges>, Ticket, Vec<Ticket>) {
  let mut lines = s.split('\n');
  let mut fields = HashMap::new();
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
  let mut nearby = Vec::new();
  loop {
    let line = lines.next().unwrap();
    if line.is_empty() {
      break;
    }
    nearby.push(parse_ticket(line));
  }
  (fields, me, nearby)
}

fn parse_ticket(s: &str) -> Ticket {
  s.split(',').map(|x| x.parse().unwrap()).collect()
}

fn parse_range(s: &str) -> RangeInclusive<u32> {
  let mut parts = s.split('-');
  let fst: u32 = parts.next().unwrap().parse().unwrap();
  let snd: u32 = parts.next().unwrap().parse().unwrap();
  fst..=snd
}

#[test]
fn t() {
  let inp = include_str!("input/d16.txt");
  assert_eq!(p1(inp), 25984);
  // assert_eq!(p2(inp), ___);
}
