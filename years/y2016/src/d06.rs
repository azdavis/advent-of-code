use helpers::HashMap;

fn run<F>(s: &str, f: F) -> String
where
  F: FnMut(HashMap<char, usize>) -> char,
{
  let len = s.lines().next().unwrap().len();
  let mut counts = vec![HashMap::<char, usize>::default(); len];
  for line in s.lines() {
    for (idx, c) in line.chars().enumerate() {
      *counts[idx].entry(c).or_default() += 1;
    }
  }
  counts.into_iter().map(f).collect()
}

pub fn p1(s: &str) -> String {
  run(s, |map| {
    map.into_iter().max_by_key(|&(_, count)| count).unwrap().0
  })
}

pub fn p2(s: &str) -> String {
  run(s, |map| {
    map.into_iter().min_by_key(|&(_, count)| count).unwrap().0
  })
}

#[test]
fn t() {
  let s = include_str!("input/d06.txt");
  assert_eq!(p1(s), "kqsdmzft");
  assert_eq!(p2(s), "tpooccyo");
}
