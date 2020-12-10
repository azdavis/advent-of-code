use std::collections::HashSet;

pub fn p1(s: &str) -> usize {
  s.split("\n\n")
    .map(|group| {
      let any_yes: HashSet<_> =
        group.split('\n').flat_map(|line| line.chars()).collect();
      any_yes.len()
    })
    .sum()
}
