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
