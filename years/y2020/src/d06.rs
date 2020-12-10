use std::collections::HashSet;

pub fn p1(s: &str) {
  let ans: usize = s
    .split("\n\n")
    .map(|group| {
      let chars_in_group: HashSet<_> =
        group.split('\n').flat_map(|line| line.chars()).collect();
      chars_in_group.len()
    })
    .sum();
  println!("{}", ans);
}
