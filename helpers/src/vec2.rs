//! 2-dimensional integer vectors.

/// A 2-dimensional integer vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[allow(missing_docs)]
pub struct Vec2 {
  pub x: i32,
  pub y: i32,
}

impl Vec2 {
  /// Returns a new Vec2.
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  /// Returns the distance to the origin.
  pub fn to_origin(&self) -> u32 {
    (self.x.abs() + self.y.abs()) as u32
  }

  /// Returns the neighbors of this.
  pub fn neighbors(self) -> [Self; 8] {
    let x = self.x;
    let y = self.y;
    [
      Self::new(x - 1, y + 1),
      Self::new(x, y + 1),
      Self::new(x + 1, y + 1),
      Self::new(x - 1, y),
      Self::new(x + 1, y),
      Self::new(x - 1, y - 1),
      Self::new(x, y - 1),
      Self::new(x + 1, y - 1),
    ]
  }
}