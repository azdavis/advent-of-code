use helpers::{Compass, HashSet};
use std::ops::ControlFlow;

enum Dir {
  Left,
  Right,
}

fn parse(s: &str) -> impl Iterator<Item = (Dir, u32)> + '_ {
  s.trim().split(", ").map(|s| {
    let mut iter = s.chars();
    let dir = match iter.next().unwrap() {
      'L' => Dir::Left,
      'R' => Dir::Right,
      c => panic!("unknown dir: {c}"),
    };
    (dir, iter.as_str().parse().unwrap())
  })
}

struct State {
  facing: Compass,
  x: i32,
  y: i32,
}

impl State {
  fn update<F>(&mut self, dir: &Dir, n: u32, f: &mut F) -> ControlFlow<()>
  where
    F: FnMut([i32; 2]) -> ControlFlow<()>,
  {
    self.facing = match dir {
      Dir::Left => self.facing.left(),
      Dir::Right => self.facing.right(),
    };
    let [dx, dy] = self.facing.dx_dy();
    for _ in 0..n {
      self.x += dx;
      self.y += dy;
      f([self.x, self.y])?;
    }
    ControlFlow::Continue(())
  }

  fn to_origin(&self) -> i32 {
    self.x.abs() + self.y.abs()
  }
}

impl Default for State {
  fn default() -> Self {
    Self {
      facing: Compass::North,
      x: 0,
      y: 0,
    }
  }
}

pub fn p1(s: &str) -> i32 {
  let mut st = State::default();
  for (dir, n) in parse(s) {
    match st.update(&dir, n, &mut |_| ControlFlow::Continue(())) {
      ControlFlow::Continue(()) => {}
      ControlFlow::Break(()) => unreachable!(),
    }
  }
  st.to_origin()
}

pub fn p2(s: &str) -> i32 {
  let mut st = State::default();
  let mut prev = HashSet::<[i32; 2]>::default();
  for (dir, n) in parse(s) {
    let res = st.update(&dir, n, &mut |p| {
      if prev.insert(p) {
        ControlFlow::Continue(())
      } else {
        ControlFlow::Break(())
      }
    });
    match res {
      ControlFlow::Continue(()) => {}
      ControlFlow::Break(()) => return st.to_origin(),
    }
  }
  panic!("no solution")
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 231);
  assert_eq!(p2(s), 147);
}
