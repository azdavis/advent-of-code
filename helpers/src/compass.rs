//! Compass directions.

/// A compass direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Compass {
  North,
  South,
  East,
  West,
}

impl Compass {
  /// Returns the result of turning left.
  pub fn left(self) -> Self {
    match self {
      Self::North => Self::West,
      Self::South => Self::East,
      Self::East => Self::North,
      Self::West => Self::South,
    }
  }

  /// Returns the result of turning right.
  pub fn right(self) -> Self {
    match self {
      Self::North => Self::East,
      Self::South => Self::West,
      Self::East => Self::South,
      Self::West => Self::North,
    }
  }

  /// Returns the result of turning around.
  pub fn reverse(self) -> Self {
    match self {
      Self::North => Self::South,
      Self::South => Self::North,
      Self::East => Self::West,
      Self::West => Self::East,
    }
  }

  /// Returns the change in x and y respectively that would result from moving
  /// forward 1 unit in this direction. North and East are positive.
  pub fn dx_dy(self) -> [i32; 2] {
    match self {
      Self::North => [0, 1],
      Self::South => [0, -1],
      Self::East => [1, 0],
      Self::West => [-1, 0],
    }
  }
}

#[test]
fn t() {
  assert_eq!(Compass::North.reverse(), Compass::South);
  assert_eq!(Compass::West.right(), Compass::North);
  assert_eq!(Compass::East.dx_dy(), [1, 0]);
}
