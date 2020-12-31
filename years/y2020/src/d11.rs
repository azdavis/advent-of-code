//! how much can we abstract?

#![allow(clippy::needless_range_loop)]
#![allow(clippy::ptr_arg)]

pub fn p1(s: &str) -> usize {
  evolve_loop(s, 4, |i, j, f, g, xs| Some(*xs.get(f(i)?)?.get(g(j)?)?))
}

pub fn p2(s: &str) -> usize {
  evolve_loop(s, 5, |mut i, mut j, f, g, xs| loop {
    i = f(i)?;
    j = g(j)?;
    let tile = *xs.get(i)?.get(j)?;
    if matches!(tile, Tile::Empty | Tile::Occupied) {
      return Some(tile);
    }
  })
}

type Grid = Vec<Vec<Tile>>;

type GetOneTile = fn(usize, usize, ChangeFn, ChangeFn, &Grid) -> Option<Tile>;

fn evolve_loop(s: &str, threshold: usize, get_one_tile: GetOneTile) -> usize {
  let mut prev = parse(s);
  loop {
    let cur = evolve_with(&prev, threshold, get_one_tile);
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

fn evolve_with(xs: &Grid, threshold: usize, get_one_tile: GetOneTile) -> Grid {
  let mut ret = xs.clone();
  for i in 0..ret.len() {
    for j in 0..ret[i].len() {
      match ret[i][j] {
        Tile::Floor => {}
        Tile::Empty => {
          let any_nearby = get_all_tiles(i, j, xs, get_one_tile)
            .into_iter()
            .any(|x| matches!(x, Tile::Occupied));
          if !any_nearby {
            ret[i][j] = Tile::Occupied;
          }
        }
        Tile::Occupied => {
          let count = get_all_tiles(i, j, xs, get_one_tile)
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

fn get_all_tiles(
  i: usize,
  j: usize,
  xs: &Grid,
  get_one_tile: GetOneTile,
) -> impl Iterator<Item = Tile> + '_ {
  FNS
    .iter()
    .filter_map(move |&(f, g)| get_one_tile(i, j, f, g, xs))
}

type ChangeFn = fn(usize) -> Option<usize>;

const FNS: [(ChangeFn, ChangeFn); 8] = [
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
  s.lines()
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

#[test]
fn t() {
  let inp = include_str!("input/d11.txt");
  assert_eq!(p1(inp), 2238);
  assert_eq!(p2(inp), 2013);
}
