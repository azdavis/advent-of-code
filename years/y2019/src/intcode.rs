mod mem;

use std::collections::VecDeque;
use std::convert::TryInto as _;

pub fn parse(s: &str) -> impl Iterator<Item = i64> + '_ {
  s.split('\n')
    .next()
    .unwrap()
    .split(',')
    .map(|x| x.parse().unwrap())
}

#[derive(Debug, Clone)]
pub struct Intcode {
  mem: mem::Mem<i64>,
  cur_addr: usize,
  relative_base: i64,
  input: VecDeque<i64>,
}

impl Intcode {
  pub fn parse(s: &str) -> Self {
    Self::new(parse(s))
  }

  pub fn new<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = i64>,
  {
    Self {
      mem: mem::Mem::new(iter),
      cur_addr: 0,
      relative_base: 0,
      input: VecDeque::new(),
    }
  }

  // for day 2
  pub fn read_zeroth(self) -> i64 {
    self.mem.read(0)
  }

  pub fn input(&mut self, inp: i64) {
    self.input.push_back(inp);
  }

  #[must_use = "the program may not be done running"]
  pub fn run(&mut self, output: &mut Vec<i64>) -> Res {
    loop {
      let cur = self.mem.read(self.cur_addr);
      let op = cur % 100;
      let modes = cur / 100;
      self.cur_addr = match op {
        1 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          let c = self.pos_arg(3, modes);
          self.mem.write(c, a + b);
          self.cur_addr + 4
        }
        2 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          let c = self.pos_arg(3, modes);
          self.mem.write(c, a * b);
          self.cur_addr + 4
        }
        3 => {
          let inp = match self.input.pop_front() {
            Some(x) => x,
            None => return Res::NeedInput,
          };
          let a = self.pos_arg(1, modes);
          self.mem.write(a, inp);
          self.cur_addr + 2
        }
        4 => {
          let a = self.arg(1, modes);
          output.push(a);
          self.cur_addr + 2
        }
        5 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          if a == 0 {
            self.cur_addr + 3
          } else {
            u(b)
          }
        }
        6 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          if a == 0 {
            u(b)
          } else {
            self.cur_addr + 3
          }
        }
        7 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          let c = self.pos_arg(3, modes);
          self.mem.write(c, if a < b { 1 } else { 0 });
          self.cur_addr + 4
        }
        8 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          let c = self.pos_arg(3, modes);
          self.mem.write(c, if a == b { 1 } else { 0 });
          self.cur_addr + 4
        }
        9 => {
          let a = self.arg(1, modes);
          self.relative_base += a;
          self.cur_addr + 2
        }
        99 => return Res::Done,
        _ => panic!("bad op: {}", op),
      };
    }
  }

  fn arg(&self, off: usize, modes: i64) -> i64 {
    let val = self.mem.read(self.cur_addr + off);
    match Mode::get(off, modes) {
      Mode::Position => self.mem.read(u(val)),
      Mode::Immediate => val,
      Mode::Relative => self.mem.read(u(self.relative_base + val)),
    }
  }

  fn pos_arg(&self, off: usize, modes: i64) -> usize {
    let val = self.mem.read(self.cur_addr + off);
    match Mode::get(off, modes) {
      Mode::Position => u(val),
      Mode::Immediate => panic!("immediate mode for positional arg"),
      Mode::Relative => u(self.relative_base + val),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Res {
  NeedInput,
  Done,
}

impl Res {
  pub fn is_done(&self) -> bool {
    matches!(*self, Self::Done)
  }
}

enum Mode {
  Position,
  Immediate,
  Relative,
}

impl Mode {
  fn get(off: usize, modes: i64) -> Self {
    let div = (1..off).fold(1, |ac, _| ac * 10);
    match (modes / div) % 10 {
      0 => Self::Position,
      1 => Self::Immediate,
      2 => Self::Relative,
      m => panic!("bad mode: {}", m),
    }
  }
}

fn u(n: i64) -> usize {
  n.try_into().unwrap()
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  #[test]
  fn cmp_8() {
    let large = super::Intcode::new(vec![
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106,
      0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105,
      1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
    ]);
    let mut output = Vec::with_capacity(1);
    for n in 0..30 {
      let mut large = large.clone();
      large.input(n);
      assert!(large.run(&mut output).is_done());
      let want = match n.cmp(&8) {
        Ordering::Less => 999,
        Ordering::Equal => 1000,
        Ordering::Greater => 1001,
      };
      let got = output.pop().unwrap();
      assert!(output.is_empty());
      assert_eq!(want, got);
    }
  }
}
