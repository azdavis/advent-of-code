use std::collections::HashSet;

pub fn p1(s: &str) -> usize {
  s.split("\n\n")
    .map(|group| {
      group
        .lines()
        .flat_map(str::chars)
        .collect::<HashSet<_>>()
        .len()
    })
    .sum()
}

pub fn p2(s: &str) -> usize {
  s.trim_end()
    .split("\n\n")
    .map(|group| {
      let mut sets = group
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>());
      let fst = sets.next().unwrap();
      sets
        .fold(fst, |ac, x| ac.intersection(&x).copied().collect())
        .len()
    })
    .sum()
}

#[test]
fn t() {
  let s = include_str!("input/d06.txt");
  assert_eq!(p1(s), 6585);
  assert_eq!(p2(s), 3276);
}
