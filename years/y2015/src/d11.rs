use helpers::HashSet;

fn valid(s: &[u8]) -> bool {
  if !s.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]) {
    return false;
  }
  if s.iter().any(|&c| matches!(c, b'i' | b'o' | b'l')) {
    return false;
  }
  let two_same: HashSet<_> = s
    .windows(2)
    .filter(|&w| (w[0] == w[1]))
    .map(|w| w[0])
    .collect();
  two_same.len() >= 2
}

fn inc(s: &mut [u8]) {
  for b in s.iter_mut().rev() {
    if *b == b'z' {
      *b = b'a';
    } else {
      *b += 1;
      break;
    }
  }
}

fn next_valid(s: &mut [u8]) {
  loop {
    inc(s);
    if valid(s) {
      return;
    }
  }
}

fn run(s: &str, rounds: usize) -> String {
  let mut bs = s.trim().to_owned().into_bytes();
  for _ in 0..rounds {
    next_valid(&mut bs);
  }
  String::from_utf8(bs).unwrap()
}

pub fn p1(s: &str) -> String {
  run(s, 1)
}

pub fn p2(s: &str) -> String {
  run(s, 2)
}

#[test]
fn t() {
  let s = include_str!("input/d11.txt");
  assert_eq!(p1(s), "hepxxyzz");
  assert_eq!(p2(s), "heqaabcc");
}
