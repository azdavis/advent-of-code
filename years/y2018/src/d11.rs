type Vec2 = [usize; 2];
type Vec3 = [usize; 3];

const GRID_DIM: usize = 300;

fn mk_grid(serial: usize) -> Vec<Vec<isize>> {
  (0..GRID_DIM)
    .map(|y| {
      (0..GRID_DIM)
        .map(|x| power([x + 1, y + 1], serial))
        .collect()
    })
    .collect()
}

fn power([x, y]: Vec2, serial: usize) -> isize {
  let rack_id = x + 10;
  let n = ((rack_id * y) + serial) * rack_id;
  let n_hundreds_digit = (n / 100) % 10;
  isize::try_from(n_hundreds_digit).unwrap() - 5
}

fn most_powerful(grid: &[Vec<isize>], sq_dim: usize) -> (Vec2, isize) {
  let dim_max = GRID_DIM - sq_dim;
  (0..dim_max)
    .flat_map(|y| {
      (0..dim_max).map(move |x| {
        let power = (0..sq_dim)
          .map(|dy| (0..sq_dim).map(|dx| grid[y + dy][x + dx]).sum::<isize>())
          .sum::<isize>();
        ([x + 1, y + 1], power)
      })
    })
    .max_by_key(|&(_, p)| p)
    .unwrap()
}

fn run_p1(serial: usize) -> (Vec2, isize) {
  most_powerful(&mk_grid(serial), 3)
}

fn get_biggest(grid: &[Vec<isize>]) -> (Vec2, isize) {
  grid
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .map(move |(x, &p)| ([x + 1, y + 1], p))
    })
    .max_by_key(|&(_, p)| p)
    .unwrap()
}

/// a naive solution would just run `most_powerful` for every sq_dim in
/// `1..GRID_DIM`, but we can do better by using previous results.
#[allow(clippy::needless_range_loop)]
fn run_p2_dp(serial: usize) -> (Vec3, isize) {
  let grid = mk_grid(serial);
  let ([x, y], mut max_power): (Vec2, isize) = get_biggest(&grid);
  let mut max_point: Vec3 = [x, y, 1usize];
  let mut prev = grid.clone();
  for sq_dim in 2..GRID_DIM {
    prev.pop().unwrap();
    for row in prev.iter_mut() {
      row.pop().unwrap();
    }
    let dim_max = GRID_DIM - sq_dim;
    for y in 0..dim_max {
      for x in 0..dim_max {
        for ya in 0..sq_dim {
          prev[y][x] += grid[y + ya][x + sq_dim - 1];
        }
        for xa in 0..sq_dim - 1 {
          prev[y][x] += grid[y + sq_dim - 1][x + xa];
        }
      }
    }
    let ([x, y], new_max_power) = get_biggest(&prev);
    if new_max_power > max_power {
      max_power = new_max_power;
      max_point = [x, y, sq_dim];
    }
  }
  (max_point, max_power)
}

pub fn p1(s: &str) -> String {
  let ([x, y], _) = run_p1(s.trim().parse().unwrap());
  format!("{},{}", x, y)
}

pub fn p2(s: &str) -> String {
  let ([x, y, sq_dim], _) = run_p2_dp(s.trim().parse().unwrap());
  format!("{},{},{}", x, y, sq_dim)
}

#[test]
fn t() {
  let s = include_str!("input/d11.txt");
  assert_eq!(p1(s), "235,22");
  assert_eq!(p2(s), "231,135,8");
}

#[test]
fn ex0() {
  assert_eq!(power([3, 5], 8), 4);
}

#[test]
fn ex1() {
  assert_eq!(run_p1(18), ([33, 45], 29));
}

#[test]
fn ex2() {
  assert_eq!(run_p1(42), ([21, 61], 30));
}

#[test]
fn ex3() {
  assert_eq!(run_p2_dp(18), ([90, 269, 16], 113));
}

#[test]
fn ex4() {
  assert_eq!(run_p2_dp(42), ([232, 251, 12], 119));
}
