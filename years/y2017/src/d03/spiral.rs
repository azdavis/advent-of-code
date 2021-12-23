#[derive(Debug)]
enum Dir {
  Rt,
  Up,
  Lt,
  Dn,
}

/// An infinite iterator of coordinates on a 2-D grid arranged in a spiral,
/// starting at the origin `[0, 0]` and going right, up, left, down repeatedly.
#[derive(Debug)]
pub(super) struct Spiral {
  level: usize,
  x: isize,
  y: isize,
  cur: usize,
  dir: Dir,
}

impl Default for Spiral {
  fn default() -> Self {
    Self {
      level: 0,
      x: 0,
      y: 0,
      cur: 1,
      dir: Dir::Rt,
    }
  }
}

impl Iterator for Spiral {
  type Item = [isize; 2];

  fn next(&mut self) -> Option<Self::Item> {
    let ret = [self.x, self.y];
    match self.dir {
      Dir::Rt => self.x += 1,
      Dir::Up => self.y -= 1,
      Dir::Lt => self.x -= 1,
      Dir::Dn => self.y += 1,
    }
    self.cur -= 1;
    if self.cur == 0 {
      let (dir, cur) = match self.dir {
        Dir::Rt => (Dir::Up, (self.level * 2) + 1),
        Dir::Up => (Dir::Lt, (self.level + 1) * 2),
        Dir::Lt => (Dir::Dn, (self.level + 1) * 2),
        Dir::Dn => {
          self.level += 1;
          (Dir::Rt, (self.level * 2) + 1)
        }
      };
      self.dir = dir;
      self.cur = cur;
    }
    Some(ret)
  }
}
