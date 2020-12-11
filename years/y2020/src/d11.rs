//! in my quest to abstract as much as possible, i added some intermediate
//! allocations by having `get_nearby` return a `Vec<Tile>` instead of e.g. a
//! `impl Iterator<Item = Tile>`.

#![allow(clippy::needless_range_loop)]
#![allow(clippy::ptr_arg)]

pub fn p1(s: &str) -> usize {
  evolve_loop(s, 4, get_nearby_p1)
}

type Grid = Vec<Vec<Tile>>;

fn evolve_loop<F>(s: &str, threshold: usize, get_nearby: F) -> usize
where
  F: Fn(usize, usize, &Grid) -> Vec<Tile> + Copy,
{
  let mut prev = parse(s);
  loop {
    let cur = evolve_with(&prev, threshold, get_nearby);
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

fn evolve_with<F>(xs: &Grid, threshold: usize, get_nearby: F) -> Grid
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
          if count >= threshold {
            ret[i][j] = Tile::Empty;
          }
        }
      }
    }
  }
  ret
}

fn get_nearby_p1(i: usize, j: usize, xs: &Grid) -> Vec<Tile> {
  FNS
    .iter()
    .filter_map(|(f, g)| f(i).zip(g(j)))
    .filter_map(move |(i, j)| Some(*xs.get(i)?.get(j)?))
    .collect()
}

type F = fn(usize) -> Option<usize>;

const FNS: [(F, F); 8] = [
  (inc, inc),
  (inc, Some),
  (inc, dec),
  (Some, inc),
  (Some, dec),
  (dec, inc),
  (dec, Some),
  (dec, dec),
];

fn inc(n: usize) -> Option<usize> {
  n.checked_add(1)
}

fn dec(n: usize) -> Option<usize> {
  n.checked_sub(1)
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
