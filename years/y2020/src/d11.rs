#![allow(clippy::needless_range_loop)]
#![allow(clippy::ptr_arg)]

pub fn p1(s: &str) -> usize {
  evolve_loop(s, |grid| evolve_with(grid, get_nearby_p1))
}

type Grid = Vec<Vec<Tile>>;

fn evolve_loop<F>(s: &str, evolve: F) -> usize
where
  F: Fn(&Grid) -> Grid,
{
  let mut prev = parse(s);
  loop {
    let cur = evolve(&prev);
    if cur == prev {
      return cur
        .iter()
        .flatten()
        .filter(|&&x| matches!(x, Tile::Occupied))
        .count();
    }
    prev = cur;
  }
}

fn evolve_with<F>(xs: &Grid, get_nearby: F) -> Grid
where
  F: Fn(usize, usize, &Grid) -> Vec<Tile>,
{
  let mut ret = xs.clone();
  for i in 0..ret.len() {
    for j in 0..ret[i].len() {
      match ret[i][j] {
        Tile::Floor => {}
        Tile::Empty => {
          let any_nearby = get_nearby(i, j, xs)
            .into_iter()
            .any(|x| matches!(x, Tile::Occupied));
          if !any_nearby {
            ret[i][j] = Tile::Occupied;
          }
        }
        Tile::Occupied => {
          let count = get_nearby(i, j, xs)
            .into_iter()
            .filter(|x| matches!(x, Tile::Occupied))
            .count();
          if count >= 4 {
            ret[i][j] = Tile::Empty;
          }
        }
      }
    }
  }
  ret
}

fn get_nearby_p1(i: usize, j: usize, xs: &Grid) -> Vec<Tile> {
  adjacent(i, j)
    .into_iter()
    .filter_map(move |(i, j)| Some(*xs.get(i)?.get(j)?))
    .collect()
}

// pretty ugly
fn adjacent(i: usize, j: usize) -> Vec<(usize, usize)> {
  let mut ret = vec![(i + 1, j + 1), (i + 1, j), (i, j + 1)];
  if let Some(i_sub_1) = i.checked_sub(1) {
    ret.push((i_sub_1, j + 1));
    ret.push((i_sub_1, j));
  }
  if let Some(j_sub_1) = j.checked_sub(1) {
    ret.push((i + 1, j_sub_1));
    ret.push((i, j_sub_1));
  }
  if let Some(i_sub_1) = i.checked_sub(1) {
    if let Some(j_sub_1) = j.checked_sub(1) {
      ret.push((i_sub_1, j_sub_1));
    }
  }
  ret
}

fn parse(s: &str) -> Grid {
  s.split('\n')
    .filter(|line| !line.is_empty())
    .map(|line| line.chars().map(Tile::parse).collect())
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
  Floor,
  Empty,
  Occupied,
}

impl Tile {
  fn parse(c: char) -> Self {
    match c {
      '.' => Self::Floor,
      'L' => Self::Empty,
      '#' => Self::Occupied,
      bad => panic!("bad char: {}", bad),
    }
  }
}
