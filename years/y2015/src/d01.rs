use std::ops::ControlFlow;

fn parse(s: &str) -> impl Iterator<Item = i32> + '_ {
  s.trim().chars().map(|c| match c {
    '(' => 1,
    ')' => -1,
    _ => panic!("unknown char: {c}"),
  })
}

pub fn p1(s: &str) -> i32 {
  parse(s).sum()
}

pub fn p2(s: &str) -> usize {
  let res = parse(s).enumerate().try_fold(0i32, |mut ac, (idx, d)| {
    ac += d;
    if ac == -1 {
      ControlFlow::Break(idx + 1)
    } else {
      ControlFlow::Continue(ac)
    }
  });
  match res {
    ControlFlow::Continue(_) => panic!("no solution"),
    ControlFlow::Break(n) => n,
  }
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 280);
  assert_eq!(p2(s), 1797);
}
