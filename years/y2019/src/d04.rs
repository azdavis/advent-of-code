use std::cmp::Ordering;

pub fn p1(s: &str) -> usize {
  let (lo, hi) = parse(s);
  (lo..=hi).filter(|&x| is_valid(x)).count()
}

pub fn p2(_: &str) -> usize {
  todo!()
}

fn is_valid(n: u32) -> bool {
  if n < 100000 || n > 999999 {
    return false;
  }
  let mut ds = helpers::digits::digits(n);
  let mut prev = ds.next().unwrap();
  let mut has_double = false;
  for d in ds {
    match d.cmp(&prev) {
      Ordering::Less => return false,
      Ordering::Equal => has_double = true,
      Ordering::Greater => {}
    }
    prev = d;
  }
  has_double
}

fn parse(s: &str) -> (u32, u32) {
  let mut parts = s.split('\n').next().unwrap().split('-');
  let lo: u32 = parts.next().unwrap().parse().unwrap();
  let hi: u32 = parts.next().unwrap().parse().unwrap();
  (lo, hi)
}

#[test]
fn t() {
  let inp = include_str!("input/d04.txt");
  assert_eq!(p1(inp), 1864);
  // assert_eq!(p2(inp), 123);
}
