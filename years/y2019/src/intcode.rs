use std::convert::TryInto as _;

pub fn parse(s: &str) -> Vec<i32> {
  s.split('\n')
    .next()
    .unwrap()
    .split(',')
    .map(|x| x.parse().unwrap())
    .collect()
}

#[derive(Debug, Clone)]
pub struct Intcode {
  inner: Vec<i32>,
  idx: usize,
}

impl Intcode {
  pub fn new(inner: Vec<i32>) -> Self {
    Self { inner, idx: 0 }
  }

  pub fn into_inner(self) -> Vec<i32> {
    self.inner
  }

  pub fn run(&mut self, input: &[i32], output: &mut Vec<i32>) {
    let mut input = input.iter().copied();
    loop {
      let cur = self.inner[self.idx];
      let op = cur % 100;
      let modes = cur / 100;
      self.idx = match op {
        1 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          let c = self.pos_arg(3, modes);
          self.inner[c] = a + b;
          self.idx + 4
        }
        2 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          let c = self.pos_arg(3, modes);
          self.inner[c] = a * b;
          self.idx + 4
        }
        3 => {
          let a = self.pos_arg(1, modes);
          self.inner[a] = input.next().unwrap();
          self.idx + 2
        }
        4 => {
          let a = self.arg(1, modes);
          output.push(a);
          self.idx + 2
        }
        5 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          if a == 0 {
            self.idx + 3
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
            self.idx + 3
          }
        }
        7 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          let c = self.pos_arg(3, modes);
          self.inner[c] = if a < b { 1 } else { 0 };
          self.idx + 4
        }
        8 => {
          let a = self.arg(1, modes);
          let b = self.arg(2, modes);
          let c = self.pos_arg(3, modes);
          self.inner[c] = if a == b { 1 } else { 0 };
          self.idx + 4
        }
        99 => break,
        _ => panic!("bad op: {}", op),
      };
    }
  }

  fn arg(&self, off: usize, modes: i32) -> i32 {
    let val = self.inner[self.idx + off];
    match mode(off, modes) {
      Mode::Position => self.inner[u(val)],
      Mode::Immediate => val,
    }
  }

  fn pos_arg(&self, off: usize, modes: i32) -> usize {
    assert!(matches!(mode(off, modes), Mode::Position));
    u(self.inner[self.idx + off])
  }
}

fn mode(off: usize, modes: i32) -> Mode {
  let div = (1..off).fold(1, |ac, _| ac * 10);
  match (modes / div) % 10 {
    0 => Mode::Position,
    1 => Mode::Immediate,
    m => panic!("bad mode: {}", m),
  }
}

enum Mode {
  Position,
  Immediate,
}

fn u(n: i32) -> usize {
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
      large.clone().run(&[n], &mut output);
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
