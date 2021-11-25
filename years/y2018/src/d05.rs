fn ok(upper: u8, lower: u8) -> bool {
  upper.is_ascii_uppercase()
    && lower.is_ascii_lowercase()
    && upper.to_ascii_lowercase() == lower
}

fn run(mut bs: Vec<u8>) -> Vec<u8> {
  'outer: loop {
    assert_eq!(bs.len() % 2, 0);
    for (idx, &b1) in bs.iter().enumerate() {
      if let Some(&b2) = idx.checked_add(1).and_then(|idx| bs.get(idx)) {
        if ok(b1, b2) || ok(b2, b1) {
          // not great for runtime
          assert_eq!(b1, bs.remove(idx));
          assert_eq!(b2, bs.remove(idx));
          continue 'outer;
        }
      }
    }
    return bs;
  }
}

pub fn p1(s: &str) -> usize {
  run(s.trim().as_bytes().to_vec()).len()
}

pub fn p2(s: &str) -> usize {
  // fool me once, shame on you. fool me twice, shame on me. (i realized i
  // needed to trim() for p1 but then floundered around on p2 for a bit before
  // looking up a solution and realizing i made the same mistake again.)
  let s = s.trim();
  (b'a'..=b'z')
    .map(|del| {
      let bs: Vec<_> = s
        .bytes()
        .filter(|b| b.to_ascii_lowercase() != del)
        .collect();
      run(bs).len()
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
  let inp = b"dabAcCaCBAcCcaDA".to_vec();
  let out = b"dabCBAcaDA".to_vec();
  assert_eq!(run(inp), out);
}

#[test]
fn ex2() {
  assert_eq!(p2("dabAcCaCBAcCcaDA"), 4);
}
