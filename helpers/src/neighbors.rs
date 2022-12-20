//! Neighbor-y operations.

use std::ops::Deref;

/// A pair of `usize`s.
pub type Coord = [usize; 2];

fn neighbors_help<'a, M, R, T, const N: usize>(
  matrix: &'a M,
  coords: [Option<Coord>; N],
) -> impl Iterator<Item = (&'a T, Coord)>
where
  M: Deref<Target = [R]>,
  R: 'a + Deref<Target = [T]>,
  T: 'a,
{
  coords.into_iter().filter_map(|xy| {
    let [x, y] = xy?;
    let v = matrix.get(y)?.get(x)?;
    Some((v, [x, y]))
  })
}

/// Returns the neighbors (up, down, left, right) of `coord` in `matrix`. The
/// iterator will be at most 4 items long.
pub fn neighbors<'a, M, R, T>(matrix: &'a M, coord: Coord) -> impl Iterator<Item = (&'a T, Coord)>
where
  M: Deref<Target = [R]>,
  R: 'a + Deref<Target = [T]>,
  T: 'a,
{
  let [x, y] = coord;
  let coords = [
    x.checked_add(1).map(|x| [x, y]),
    x.checked_sub(1).map(|x| [x, y]),
    y.checked_add(1).map(|y| [x, y]),
    y.checked_sub(1).map(|y| [x, y]),
  ];
  neighbors_help(matrix, coords)
}

/// Returns the neighbors (including diagonal neighbors) of `coord` in `matrix`.
/// The iterator will be at most 8 items long.
pub fn neighbors_diag<'a, M, R, T>(
  matrix: &'a M,
  coord: Coord,
) -> impl Iterator<Item = (&'a T, Coord)>
where
  M: Deref<Target = [R]>,
  R: 'a + Deref<Target = [T]>,
  T: 'a,
{
  let [x, y] = coord;
  let x_add_1 = x.checked_add(1);
  let x_sub_1 = x.checked_sub(1);
  let y_add_1 = y.checked_add(1);
  let y_sub_1 = y.checked_sub(1);
  let coords = [
    x_add_1.map(|x| [x, y]),
    x_sub_1.map(|x| [x, y]),
    y_add_1.map(|y| [x, y]),
    y_sub_1.map(|y| [x, y]),
    x_add_1.and_then(|x| y_add_1.map(|y| [x, y])),
    x_sub_1.and_then(|x| y_add_1.map(|y| [x, y])),
    x_add_1.and_then(|x| y_sub_1.map(|y| [x, y])),
    x_sub_1.and_then(|x| y_sub_1.map(|y| [x, y])),
  ];
  neighbors_help(matrix, coords)
}

/// A signed coordinate.
pub type SignedCoord = [isize; 2];

/// Returns the signed neighbors of the coord.
pub fn signed_neighbors(coord: SignedCoord) -> [SignedCoord; 8] {
  let [x, y] = coord;
  [
    [x - 1, y - 1],
    [x, y - 1],
    [x + 1, y - 1],
    [x - 1, y],
    [x + 1, y],
    [x - 1, y + 1],
    [x, y + 1],
    [x + 1, y + 1],
  ]
}
