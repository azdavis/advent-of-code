fn react(upper: u8, lower: u8) -> bool {
  upper.is_ascii_uppercase()
    && lower.is_ascii_lowercase()
    && upper.to_ascii_lowercase() == lower
}

fn run(bs: &[u8]) -> Vec<u8> {
  let mut ret = Vec::with_capacity(bs.len());
  assert_eq!(bs.len() % 2, 0);
  for &b2 in bs.iter() {
    let should_rm = ret
      .last()
      .map_or(false, |&b1| react(b1, b2) || react(b2, b1));
    if should_rm {
      ret.pop().unwrap();
    } else {
      ret.push(b2);
    }
  }
  ret.shrink_to_fit();
  ret
}

pub fn p1(s: &str) -> usize {
  run(s.trim().as_bytes()).len()
}

pub fn p2(s: &str) -> usize {
  // fool me once, shame on you. fool me twice, shame on me. (i realized i
  // needed to trim() for p1 but then floundered around on p2 for a bit before
  // looking up a solution and realizing i made the same mistake again.)
  let s = s.trim();
  (b'a'..=b'z')
    .map(|b_rm| {
      let bs: Vec<_> = s
        .bytes()
        .filter(|b| b.to_ascii_lowercase() != b_rm)
        .collect();
      run(&bs).len()
    })
    .min()
    .unwrap()
}

#[test]
fn t() {
  let s = include_str!("input/d05.txt");
  assert_eq!(p1(s), 9526);
  assert_eq!(p2(s), 6694);
}

#[test]
fn ex1() {
  assert_eq!(run(b"dabAcCaCBAcCcaDA"), b"dabCBAcaDA");
}

#[test]
fn ex2() {
  assert_eq!(p2("dabAcCaCBAcCcaDA"), 4);
}
