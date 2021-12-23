use helpers::Counter;

fn run<F>(s: &str, f: F) -> String
where
  F: FnMut(Counter<char>) -> char,
{
  let len = s.lines().next().unwrap().len();
  let mut counts = vec![Counter::<char>::default(); len];
  for line in s.lines() {
    for (idx, c) in line.chars().enumerate() {
      counts[idx].inc(c);
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
