// use md5::compute;
use std::fmt::{Error, Write as _};

const LEN: usize = 8;

fn run<F>(s: &str, f: &mut F) -> Result<String, Error>
where
  F: FnMut(&mut Vec<Option<char>>, char, char),
{
  let mut idx = 0usize;
  let mut inp = String::new();
  let mut out = String::new();
  let mut ret = vec![None::<char>; LEN];
  while ret.iter().any(Option::is_none) {
    write!(&mut inp, "{}{}", s, idx)?;
    write!(&mut out, "{:x}", md5::compute(&inp))?;
    let mut chars = out.chars();
    if chars.by_ref().take(5).all(|c| c == '0') {
      f(&mut ret, chars.next().unwrap(), chars.next().unwrap());
    }
    inp.clear();
    out.clear();
    idx += 1;
  }
  Ok(ret.into_iter().map(Option::unwrap).collect())
}

pub fn p1(s: &str) -> Result<String, Error> {
  let mut idx = 0usize;
  run(s.trim(), &mut |s, c, _| {
    s[idx] = Some(c);
    idx += 1;
  })
}

pub fn p2(s: &str) -> Result<String, Error> {
  run(s.trim(), &mut |s, pos, c| {
    let idx: usize = pos.to_digit(16).unwrap().try_into().unwrap();
    if let Some(None) = s.get(idx) {
      s[idx] = Some(c);
    }
  })
}

#[test]
fn t() {
  let s = include_str!("input/d05.txt");
  assert_eq!(p1(s).unwrap(), "c6697b55");
  assert_eq!(p2(s).unwrap(), "8c35d1ab");
}
