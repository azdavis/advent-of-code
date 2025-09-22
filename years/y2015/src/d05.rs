const P1_BAD: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn run(s: &str, f: fn(&str) -> bool) -> usize {
  s.lines().filter(|&s| f(s)).count()
}

pub fn p1(s: &str) -> usize {
  run(s, |line| {
    line.chars().filter(|c| VOWELS.contains(c)).count() >= 3
      && line.as_bytes().windows(2).any(|w| w[0] == w[1])
      && P1_BAD.into_iter().all(|s| !line.contains(s))
  })
}

pub fn p2(s: &str) -> usize {
  run(s, |line| {
    let bs = line.as_bytes();
    bs.windows(2).enumerate().any(|(i1, w1)| {
      bs.windows(2)
        .enumerate()
        .any(|(i2, w2)| w1 == w2 && i1.abs_diff(i2) >= 2)
    }) && bs.windows(3).any(|w| w[0] == w[2])
  })
}

#[test]
fn t() {
  let s = include_str!("input/d05.txt");
  assert_eq!(p1(s), 236);
  assert_eq!(p2(s), 51);
}
