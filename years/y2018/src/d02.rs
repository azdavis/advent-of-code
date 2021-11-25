use helpers::HashMap;

pub fn p1(s: &str) -> usize {
  let mut has_2 = 0usize;
  let mut has_3 = 0usize;
  for line in s.lines() {
    let mut map = HashMap::<char, usize>::default();
    for c in line.chars() {
      *map.entry(c).or_default() += 1;
    }
    if map.values().any(|&v| v == 2) {
      has_2 += 1;
    }
    if map.values().any(|&v| v == 3) {
      has_3 += 1;
    }
  }
  has_2 * has_3
}

pub fn p2(s: &str) -> String {
  let words: Vec<Vec<_>> =
    s.lines().map(|line| line.chars().collect()).collect();
  for (idx, w1) in words.iter().enumerate() {
    for w2 in words[idx..].iter() {
      let diff = w1
        .iter()
        .zip(w2.iter())
        .filter(|&(&c1, &c2)| c1 != c2)
        .count();
      if diff == 1 {
        return w1
          .iter()
          .zip(w2.iter())
          .filter_map(|(&c1, &c2)| (c1 == c2).then(|| c1))
          .collect();
      }
    }
  }
  panic!("no solution")
}

#[test]
fn t() {
  let s = include_str!("input/d02.txt");
  assert_eq!(p1(s), 6225);
  assert_eq!(p2(s), "revtaubfniyhsgxdoajwkqilp");
}
