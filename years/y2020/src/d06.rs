use std::collections::HashSet;

pub fn p1(s: &str) -> usize {
  s.split("\n\n")
    .map(|group| {
      group
        .split('\n')
        .flat_map(|line| line.chars())
        .collect::<HashSet<_>>()
        .len()
    })
    .sum()
}

pub fn p2(s: &str) -> usize {
  s.split("\n\n")
    .map(|group| {
      let mut sets = group.split('\n').filter_map(|line| {
        if line.is_empty() {
          None
        } else {
          Some(line.chars().collect::<HashSet<_>>())
        }
      });
      let fst = sets.next().unwrap();
      sets
        .fold(fst, |ac, x| ac.intersection(&x).copied().collect())
        .len()
    })
    .sum()
}
