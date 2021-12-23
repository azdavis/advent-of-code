use helpers::{static_regex, Counter};
use std::cmp::Reverse;

static_regex!(RE = r"^([a-z\-]+)-(\d+)\[([a-z]+)\]$");

fn parse(s: &str) -> impl Iterator<Item = (&str, usize, &str)> + '_ {
  s.lines().map(|line| {
    let caps = RE.captures(line).unwrap();
    let name = caps.get(1).unwrap().as_str();
    let id: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let ck_sum = caps.get(3).unwrap().as_str();
    (name, id, ck_sum)
  })
}

pub fn p1(s: &str) -> usize {
  parse(s)
    .filter_map(|(name, id, ck_sum)| {
      let mut counts = Counter::<char>::default();
      for c in name.chars() {
        if c == '-' {
          continue;
        };
        counts.inc(c);
      }
      let mut order: Vec<_> = counts
        .into_iter()
        .map(|(c, count)| (Reverse(count), c))
        .collect();
      order.sort_unstable();
      ck_sum
        .chars()
        .zip(order)
        .all(|(a, (_, b))| a == b)
        .then(|| id)
    })
    .sum()
}

pub fn p2(s: &str) -> usize {
  parse(s)
    .map(|(name, id, _)| {
      let shifted_name: String = name
        .bytes()
        .map(move |c| {
          if c == b'-' {
            ' '
          } else {
            let n: u8 = ((usize::from(c - b'a') + id) % 26).try_into().unwrap();
            char::from(n + b'a')
          }
        })
        .collect();
      (shifted_name, id)
    })
    .find_map(|(name, id)| (name == "northpole object storage").then(|| id))
    .unwrap()
}

#[test]
fn t() {
  let s = include_str!("input/d04.txt");
  assert_eq!(p1(s), 361724);
  assert_eq!(p2(s), 482);
}
