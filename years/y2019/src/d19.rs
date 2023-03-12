use crate::intcode::Intcode;

struct Beam {
  prog: Intcode,
  out: Vec<i64>,
}

impl Beam {
  fn new(s: &str) -> Self {
    Self {
      prog: Intcode::parse(s),
      out: Vec::with_capacity(1),
    }
  }

  fn affects(&mut self, x: i64, y: i64) -> bool {
    assert!(self.out.is_empty());
    let mut prog = self.prog.clone();
    prog.input(x);
    prog.input(y);
    assert!(prog.run(&mut self.out).is_done());
    match self.out.pop().unwrap() {
      0 => false,
      1 => true,
      out => panic!("bad out: {out}"),
    }
  }
}

pub fn p1(s: &str) -> usize {
  let mut beam = Beam::new(s);
  (0i64..50)
    .map(|x| (0i64..50).filter(|&y| beam.affects(x, y)).count())
    .sum()
}

pub fn p2(s: &str) -> i64 {
  let mut beam = Beam::new(s);
  let mut x = 100i64;
  let mut y = 0;
  while !beam.affects(x, y) {
    y += 1;
  }
  loop {
    if beam.affects(x - 99, y + 99) {
      return (x - 99) * 10000 + y;
    }
    y += 1;
    while beam.affects(x + 1, y) {
      x += 1;
    }
  }
}

#[test]
fn t() {
  let s = include_str!("input/d19.txt");
  assert_eq!(p1(s), 197);
  assert_eq!(p2(s), 9_181_022);
}
