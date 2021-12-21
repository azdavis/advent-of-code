fn run(s: &str, f: fn(i16) -> i16) -> usize {
  let mut ns: Vec<i16> = s.lines().map(|it| it.parse().unwrap()).collect();
  let mut idx = 0i16;
  let mut ret = 0usize;
  loop {
    let jmp = match ns.get_mut(usize::try_from(idx).unwrap()) {
      None => return ret,
      Some(it) => it,
    };
    idx += *jmp;
    *jmp = f(*jmp);
    ret += 1;
  }
}

pub fn p1(s: &str) -> usize {
  run(s, |idx| idx + 1)
}

pub fn p2(s: &str) -> usize {
  run(s, |idx| if idx >= 3 { idx - 1 } else { idx + 1 })
}

#[test]
fn t() {
  let s = include_str!("input/d05.txt");
  assert_eq!(p1(s), 315613);
  assert_eq!(p2(s), 22570529);
}
