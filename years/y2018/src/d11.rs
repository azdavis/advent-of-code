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

fn run_p2(serial: usize) -> (Vec3, isize) {
  let grid = mk_grid(serial);
  (1..GRID_DIM)
    .map(|sq_dim| {
      let ([x, y], p) = most_powerful(&grid, sq_dim);
      ([x, y, sq_dim], p)
    })
    .max_by_key(|&(_, p)| p)
    .unwrap()
}

pub fn p1(s: &str) -> String {
  let ([x, y], _) = run_p1(s.trim().parse().unwrap());
  format!("{},{}", x, y)
}

pub fn p2(s: &str) -> String {
  let ([x, y, sq_dim], _) = run_p2(s.trim().parse().unwrap());
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
  assert_eq!(run_p2(18), ([90, 269, 16], 113));
}

#[test]
fn ex4() {
  assert_eq!(run_p2(42), ([232, 251, 12], 119));
}
