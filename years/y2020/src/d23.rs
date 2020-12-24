use helpers::digits::to_char;
use std::collections::HashMap;

pub fn p1(s: &str) -> String {
  go_p1(s, 100)
}

pub fn p2(s: &str) -> u64 {
  let (mut cups, first) = mk_map_p2(s);
  go(&mut cups, first, 10_000_000);
  let n1 = cups[&1];
  let n2 = cups[&n1];
  u64::from(n1) * u64::from(n2)
}

fn go_p1(s: &str, rounds: usize) -> String {
  let (mut cups, first) = mk_map_p1(s);
  go(&mut cups, first, rounds);
  let mut ret = String::with_capacity(cups.len() - 1);
  let mut cur = cups[&1];
  while cur != 1 {
    ret.push(to_char(cur));
    cur = cups[&cur];
  }
  ret
}

fn mk_map_p1(s: &str) -> (HashMap<u32, u32>, u32) {
  let cups = parse(s);
  let first = *cups.first().unwrap();
  let last = *cups.last().unwrap();
  let map: HashMap<_, _> = cups
    .windows(2)
    .map(|xs| (xs[0], xs[1]))
    .chain(std::iter::once((last, first)))
    .collect();
  (map, first)
}

const MAX_ELEM_P2: u32 = 1_000_000;

fn mk_map_p2(s: &str) -> (HashMap<u32, u32>, u32) {
  let cups = parse(s);
  let first = *cups.first().unwrap();
  let last = *cups.last().unwrap();
  let max = *cups.iter().max().unwrap();
  let map: HashMap<_, _> = cups
    .windows(2)
    .map(|xs| (xs[0], xs[1]))
    .chain(std::iter::once((last, max + 1)))
    .chain((max + 1..MAX_ELEM_P2).map(|x| (x, x + 1)))
    .chain(std::iter::once((MAX_ELEM_P2, first)))
    .collect();
  (map, first)
}

// take advantage of the fact that there are no duplicate elements to avoid both
// indexing logic and linked lists, and just map each element to its successor.
fn go(cups: &mut HashMap<u32, u32>, mut cur: u32, rounds: usize) {
  assert!(cups.len() >= 5);
  let min_cup = *cups.values().min().unwrap();
  let max_cup = *cups.values().max().unwrap();
  for _ in 0..rounds {
    let pick_up: Vec<_> = (0..3)
      .map(|_| {
        let to_rm = cups[&cur];
        let next = cups[&to_rm];
        cups.insert(cur, next);
        to_rm
      })
      .collect();
    let mut dest = cur - 1;
    loop {
      if dest < min_cup {
        dest = max_cup;
      }
      if !pick_up.contains(&dest) {
        break;
      }
      dest -= 1;
    }
    for n in pick_up.into_iter().rev() {
      let next = cups.insert(dest, n).unwrap();
      cups.insert(n, next);
    }
    cur = cups[&cur];
  }
}

fn parse(s: &str) -> Vec<u32> {
  s.trim_end()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect()
}

#[test]
fn t() {
  let inp = include_str!("input/d23.txt");
  assert_eq!(p1(inp), "89372645");
  assert_eq!(p2(inp), 21273394210);
}

#[cfg(test)]
mod tests {
  use super::{go_p1, mk_map_p1, mk_map_p2, p2, MAX_ELEM_P2};
  use helpers::maplit::hashmap;

  #[test]
  fn t_p1() {
    assert_eq!(go_p1("389125467", 10), "92658374");
    assert_eq!(go_p1("389125467", 100), "67384529");
  }

  #[test]
  fn t_p2() {
    assert_eq!(p2("389125467"), 149245887792);
  }

  #[test]
  fn t_mk_map_p1() {
    let (map, fst) = mk_map_p1("45312");
    let want = hashmap![
      4 => 5,
      5 => 3,
      3 => 1,
      1 => 2,
      2 => 4,
    ];
    assert_eq!(map, want);
    assert_eq!(fst, 4);
  }

  #[test]
  fn t_mk_map_p2() {
    let (map, fst) = mk_map_p2("3215674");
    assert_eq!(map.len(), MAX_ELEM_P2 as usize);
    assert_eq!(map[&3], 2);
    assert_eq!(map[&2], 1);
    assert_eq!(map[&1], 5);
    assert_eq!(map[&5], 6);
    assert_eq!(map[&6], 7);
    assert_eq!(map[&7], 4);
    assert_eq!(map[&4], 8);
    for i in 8..MAX_ELEM_P2 {
      assert_eq!(map[&i], i + 1);
    }
    assert_eq!(map[&MAX_ELEM_P2], 3);
    assert_eq!(fst, 3);
  }
}
