use std::collections::hash_map::{Entry, HashMap};

pub fn p1(s: &str) -> usize {
  go(s, 4)
}

fn go(s: &str, n: usize) -> usize {
  let s = s.as_bytes();
  let mut map = HashMap::<u8, usize>::new();
  assert!(s.len() >= n);
  for &b in s.iter().take(n) {
    *map.entry(b).or_default() += 1;
  }
  for (idx, &b) in s.iter().enumerate().skip(n) {
    if map.len() == n {
      return idx;
    }
    let mut oc = match map.entry(s[idx - n]) {
      Entry::Occupied(x) => x,
      Entry::Vacant(_) => unreachable!(),
    };
    *oc.get_mut() -= 1;
    if *oc.get() == 0 {
      oc.remove();
    }
    *map.entry(b).or_default() += 1;
  }
  unreachable!()
}

pub fn p2(s: &str) -> usize {
  go(s, 14)
}

#[test]
fn t() {
  let s = include_str!("input/d06.txt");
  assert_eq!(p1(s), 1833);
  assert_eq!(p2(s), 0);
}
