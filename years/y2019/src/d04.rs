use std::cmp::Ordering;

pub fn p1(s: &str) -> usize {
  let [lo, hi] = parse(s);
  (lo..=hi).filter(|&n| is_valid_p1(n)).count()
}

pub fn p2(s: &str) -> usize {
  let [lo, hi] = parse(s);
  (lo..=hi).filter(|&n| is_valid_p2(n)).count()
}

fn is_valid_p1(n: u32) -> bool {
  if !(100_000..1_000_000).contains(&n) {
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

fn is_valid_p2(n: u32) -> bool {
  if !(100_000..1_000_000).contains(&n) {
    return false;
  }
  let mut ds = helpers::digits::digits(n);
  let mut prev = ds.next().unwrap();
  let mut has_double = false;
  let mut run = Run::One;
  for d in ds {
    run = match d.cmp(&prev) {
      Ordering::Less => return false,
      Ordering::Equal => match run {
        Run::One => Run::Two,
        Run::Two | Run::Big => Run::Big,
      },
      Ordering::Greater => {
        if matches!(run, Run::Two) {
          has_double = true;
        }
        Run::One
      }
    };
    prev = d;
  }
  has_double || matches!(run, Run::Two)
}

enum Run {
  One,
  Two,
  Big,
}

fn parse(s: &str) -> [u32; 2] {
  let mut parts = s.trim_end().split('-');
  let lo: u32 = parts.next().unwrap().parse().unwrap();
  let hi: u32 = parts.next().unwrap().parse().unwrap();
  [lo, hi]
}

#[test]
fn t() {
  let s = include_str!("input/d04.txt");
  assert_eq!(p1(s), 1864);
  assert_eq!(p2(s), 1258);
}
