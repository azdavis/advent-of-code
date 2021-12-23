use helpers::neighbors::{neighbors_diag, Coord};
use helpers::HashSet;

type Grid = Vec<Vec<u8>>;

fn parse(s: &str) -> Grid {
  s.lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect()
    })
    .collect()
}

fn evolve(grid: &mut Grid) -> usize {
  for row in grid.iter_mut() {
    for sq in row.iter_mut() {
      *sq += 1;
    }
  }
  let mut flashed_overall = HashSet::<Coord>::default();
  loop {
    let flashed: Vec<Coord> = grid
      .iter()
      .enumerate()
      .flat_map(|(y, row)| {
        row.iter().enumerate().map(move |(x, &sq)| ([x, y], sq))
      })
      .filter_map(|(xy, sq)| {
        (!flashed_overall.contains(&xy) && sq > 9).then(|| xy)
      })
      .collect();
    if flashed.is_empty() {
      break;
    }
    let increased: Vec<Coord> = flashed
      .iter()
      .flat_map(|&coord| neighbors_diag(grid, coord))
      .map(|it| it.1)
      .collect();
    for [x, y] in increased {
      grid[y][x] += 1;
    }
    flashed_overall.extend(flashed);
  }
  let ret = flashed_overall.len();
  for [x, y] in flashed_overall {
    grid[y][x] = 0;
  }
  ret
}

pub fn p1(s: &str) -> usize {
  let mut grid = parse(s);
  let mut ret = 0usize;
  for _ in 0..100 {
    ret += evolve(&mut grid);
  }
  ret
}

pub fn p2(s: &str) -> usize {
  let mut grid = parse(s);
  let n = grid.iter().flatten().count();
  let mut idx = 1usize;
  loop {
    if n == evolve(&mut grid) {
      return idx;
    }
    idx += 1
  }
}

#[test]
fn t() {
  let s = include_str!("input/d11.txt");
  assert_eq!(p1(s), 1601);
  assert_eq!(p2(s), 368);
}

#[test]
fn ex1() {
  let s = include_str!("input/d11_ex1.txt");
  assert_eq!(p1(s), 1656);
  assert_eq!(p2(s), 195);
}
