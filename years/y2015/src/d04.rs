use std::fmt::{Error, Write as _};

fn run(s: &str, zeroes: usize) -> Result<usize, Error> {
  let mut ret = 0usize;
  let mut inp = String::with_capacity(s.len());
  let mut out = String::with_capacity(32);
  loop {
    write!(&mut inp, "{}{}", s, ret)?;
    write!(&mut out, "{:x}", md5::compute(&inp))?;
    if out.bytes().take(zeroes).all(|b| b == b'0') {
      return Ok(ret);
    }
    inp.clear();
    out.clear();
    ret += 1;
  }
}

pub fn p1(s: &str) -> usize {
  run(s.trim(), 5).unwrap()
}

pub fn p2(s: &str) -> usize {
  run(s.trim(), 6).unwrap()
}

#[test]
fn t() {
  let s = include_str!("input/d04.txt");
  assert_eq!(p1(s), 282749);
  assert_eq!(p2(s), 9962624);
}
