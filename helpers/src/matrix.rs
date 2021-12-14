//! Matrix-y operations.

use std::ops::Deref;

/// Rotates the matrix to the left. Requires there exists c such that for all x
/// in xs, x.len() == c.
pub fn rotate_left<T>(xs: &[Vec<T>]) -> Vec<Vec<T>>
where
  T: Copy,
{
  (0..xs.first().map_or(0, Vec::len))
    .map(|j| xs.iter().rev().map(|row| row[j]).collect())
    .collect()
}

#[test]
fn t_rotate_left() {
  let e: Vec<Vec<u32>> = vec![];
  let es: &[Vec<u32>] = &[];
  assert_eq!(rotate_left(es), e);
  assert_eq!(rotate_left(&[vec![1]]), [[1]]);
  assert_eq!(rotate_left(&[vec![1, 2], vec![3, 4]]), [[3, 1], [4, 2]]);
  assert_eq!(rotate_left(&[vec![1, 2]]), [[1], [2]]);
  assert_eq!(rotate_left(&[vec![1], vec![2]]), [[2, 1]]);
  assert_eq!(
    rotate_left(&[vec![1, 2, 3], vec![4, 5, 6]]),
    [[4, 1], [5, 2], [6, 3]]
  );
  assert_eq!(
    rotate_left(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
    [[7, 4, 1], [8, 5, 2], [9, 6, 3]]
  );
}

/// Transposes the matrix. Requires there exists c such that for all x in xs,
/// x.len() == c.
pub fn transpose<T>(xs: &[Vec<T>]) -> Vec<Vec<T>>
where
  T: Copy,
{
  (0..xs.first().map_or(0, Vec::len))
    .map(|j| xs.iter().map(|row| row[j]).collect())
    .collect()
}

#[test]
fn t_transpose() {
  let e: Vec<Vec<u32>> = vec![];
  let es: &[Vec<u32>] = &[];
  assert_eq!(transpose(es), e);
  assert_eq!(transpose(&[vec![1]]), [[1]]);
  assert_eq!(transpose(&[vec![1, 2], vec![3, 4]]), [[1, 3], [2, 4]]);
  assert_eq!(transpose(&[vec![1, 2]]), [[1], [2]]);
  assert_eq!(transpose(&[vec![1], vec![2]]), [[1, 2]]);
  assert_eq!(
    transpose(&[vec![1, 2, 3], vec![4, 5, 6]]),
    [[1, 4], [2, 5], [3, 6]]
  );
  assert_eq!(
    transpose(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
    [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
  );
}

/// Gets the topmost row.
pub fn top<T>(xs: &[Vec<T>]) -> Vec<T>
where
  T: Copy,
{
  xs.first().unwrap().clone()
}

/// Gets the bottommost row.
pub fn bot<T>(xs: &[Vec<T>]) -> Vec<T>
where
  T: Copy,
{
  xs.last().unwrap().clone()
}

/// Gets the leftmost column.
pub fn left<T>(xs: &[Vec<T>]) -> Vec<T>
where
  T: Copy,
{
  xs.iter().map(|x| *x.first().unwrap()).collect()
}

/// Gets the rightmost column.
pub fn right<T>(xs: &[Vec<T>]) -> Vec<T>
where
  T: Copy,
{
  xs.iter().map(|x| *x.last().unwrap()).collect()
}

/// A pair of usizes.
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
pub fn neighbors<'a, M, R, T>(
  matrix: &'a M,
  coord: Coord,
) -> impl Iterator<Item = (&'a T, Coord)>
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
