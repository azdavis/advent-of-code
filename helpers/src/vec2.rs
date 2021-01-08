//! 2-dimensional integer vectors.

use crate::compass::Compass;

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

  /// Returns the neighbors of this. Each neighbor also has the compass
  /// direction one needs to move from this to get to that neighbor.
  pub fn neighbors(self) -> [(Compass, Self); 4] {
    let x = self.x;
    let y = self.y;
    [
      (Compass::North, Self::new(x, y + 1)),
      (Compass::West, Self::new(x - 1, y)),
      (Compass::East, Self::new(x + 1, y)),
      (Compass::South, Self::new(x, y - 1)),
    ]
  }
}
