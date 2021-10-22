use helpers::digits::to_char;

pub fn p1(s: &str) -> String {
  go_p1(s, NUM_ROUNDS)
}

pub fn p2(s: &str) -> String {
  let orig_ns = parse(s);
  let start = orig_ns[..7]
    .iter()
    .fold(0, |ac, &x| (ac * 10) + usize::from(x));
  let end = orig_ns.len() * NUM_COPIES;
  assert!(start >= (end + 1) / 2);
  let mut ns: Vec<_> =
    (start..end).map(|i| orig_ns[i % orig_ns.len()]).collect();
  for _ in 0..NUM_ROUNDS {
    let mut sum = 0;
    for n in ns.iter_mut().rev() {
      sum += u32::from(*n);
      *n = (sum % 10) as u8;
    }
  }
  take_digits(ns)
}

const NUM_ROUNDS: usize = 100;
const NUM_COPIES: usize = 10_000;
const NUM_DIGITS: usize = 8;

fn take_digits(ns: Vec<u8>) -> String {
  let mut ret = String::with_capacity(NUM_DIGITS);
  for n in ns.into_iter().take(NUM_DIGITS) {
    ret.push(to_char(u32::from(n)));
  }
  ret
}

fn go_p1(s: &str, rounds: usize) -> String {
  let mut ns = parse(s);
  for _ in 0..rounds {
    let ns_clone = ns.clone();
    for (row, digit_ref) in ns.iter_mut().enumerate() {
      let sum: i64 = ns_clone
        .iter()
        .enumerate()
        .map(|(col, &digit)| i64::from(digit) * pat_digit(row, col))
        .sum();
      *digit_ref = i64_to_u8(sum.abs() % 10);
    }
  }
  take_digits(ns)
}

const BASE: [i64; 4] = [0, 1, 0, -1];

fn pat_digit(row: usize, col: usize) -> i64 {
  let r = row + 1;
  let big_pat_len = r * BASE.len();
  let rem = (col + 1) % big_pat_len;
  BASE[rem / r]
}

fn parse(s: &str) -> Vec<u8> {
  s.trim_end()
    .chars()
    .map(|x| u32_to_u8(x.to_digit(10).unwrap()))
    .collect()
}

fn u32_to_u8(n: u32) -> u8 {
  n.try_into().unwrap()
}

fn i64_to_u8(n: i64) -> u8 {
  n.try_into().unwrap()
}

#[test]
fn t_pat_digit() {
  let want = [
    [1, 0, -1, 0, 1, 0, -1, 0],
    [0, 1, 1, 0, 0, -1, -1, 0],
    [0, 0, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 0],
    [0, 0, 0, 0, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 1],
  ];
  let n = want.len();
  let got: Vec<Vec<_>> = (0..n)
    .map(|row| (0..n).map(|col| pat_digit(row, col)).collect())
    .collect();
  assert_eq!(got, want);
}

#[test]
fn t_p1() {
  assert_eq!(go_p1("12345678", 1), "48226158");
  assert_eq!(go_p1("12345678", 2), "34040438");
  assert_eq!(go_p1("12345678", 3), "03415518");
  assert_eq!(go_p1("12345678", 4), "01029498");
}

#[test]
fn t() {
  let s = include_str!("input/d16.txt");
  assert_eq!(p1(s), "49254779");
  assert_eq!(p2(s), "55078585");
}
