pub fn p1(s: &str) -> i32 {
  s.lines().map(|line| line.parse::<i32>().unwrap()).sum()
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 516);
  // assert_eq!(p2(s), ___);
}
