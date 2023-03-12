pub fn p1(s: &str) -> u32 {
  get_all(s).into_iter().max().unwrap()
}

pub fn p2(s: &str) -> u32 {
  let mut all = get_all(s);
  all.sort_unstable();
  all.into_iter().rev().take(3).sum()
}

fn get_all(s: &str) -> Vec<u32> {
  let mut cur = 0u32;
  let mut all = Vec::<u32>::new();
  for line in s.lines() {
    if line.is_empty() {
      all.push(cur);
      cur = 0;
    } else {
      cur += line.parse::<u32>().unwrap();
    }
  }
  all
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 66487);
  assert_eq!(p2(s), 197_301);
}
