use helpers::matrix::{bot, left, right, rotate_left, top, transpose};
use helpers::once_cell::sync::Lazy;
use helpers::regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn p1(s: &str) -> u64 {
  let board = go(s);
  let top = board.first().unwrap();
  let bot = board.last().unwrap();
  top.first().unwrap().0
    * top.last().unwrap().0
    * bot.first().unwrap().0
    * bot.last().unwrap().0
}

pub fn p2(s: &str) -> u32 {
  todo!()
}

fn go(s: &str) -> Board {
  let tiles = parse(s);
  let n = sqrt(tiles.len());
  let tiles: Tiles = tiles
    .into_iter()
    .map(|(n, t0)| (n, get_all_translations(t0)))
    .collect();
  let mut edges = Edges::new();
  for (&id_a, tile_variants) in tiles.iter() {
    for (id_b, tile) in tile_variants.iter().enumerate() {
      for &(f, dir) in FNS.iter() {
        edges
          .entry((f(&tile), dir))
          .or_default()
          .insert((id_a, id_b));
      }
    }
  }
  // no more mutation
  let edges = edges;
  for &id_a in tiles.keys() {
    let mut tiles = tiles.clone();
    let mut candidates: Vec<(Board, Tiles)> = tiles
      .remove(&id_a)
      .unwrap()
      .into_iter()
      .map(|tile| (vec![vec![(id_a, tile)]], tiles.clone()))
      .collect();
    for row in 0..n {
      if row != 0 {
        for (board, _) in candidates.iter_mut() {
          board.push(vec![]);
        }
      }
      for col in 0..n {
        if row == 0 && col == 0 {
          continue;
        }
        candidates = candidates
          .into_iter()
          .flat_map(|(board, tiles)| expand(&edges, board, tiles, row, col))
          .collect();
      }
    }
    if let Some((board, _)) = candidates.pop() {
      return board;
    }
  }
  panic!("no solution")
}

type Tile = Vec<Vec<Pixel>>;
type Tiles = HashMap<u64, Vec<Tile>>;
type Board = Vec<Vec<(u64, Tile)>>;
type Edges = HashMap<(Vec<Pixel>, Dir), HashSet<(u64, usize)>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pixel {
  B,
  W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
  Top,
  Bot,
  Left,
  Right,
}

#[allow(clippy::type_complexity)]
const FNS: [(for<'a> fn(&'a [Vec<Pixel>]) -> Vec<Pixel>, Dir); 4] = [
  (top, Dir::Top),
  (bot, Dir::Bot),
  (left, Dir::Left),
  (right, Dir::Right),
];

fn sqrt(n: usize) -> usize {
  let mut ret = 1;
  loop {
    match (ret * ret).cmp(&n) {
      Ordering::Less => ret += 1,
      Ordering::Equal => return ret,
      Ordering::Greater => panic!("no exact square root for {}", n),
    }
  }
}

fn expand(
  edges: &Edges,
  board: Board,
  tiles: Tiles,
  row: usize,
  col: usize,
) -> impl Iterator<Item = (Board, Tiles)> {
  assert_eq!(board.len(), row + 1);
  assert_eq!(board[row].len(), col);
  let top = row
    .checked_sub(1)
    .map(|row| edges.get(&(bot(&board[row][col].1), Dir::Top)).unwrap());
  let left = col
    .checked_sub(1)
    .map(|col| edges.get(&(right(&board[row][col].1), Dir::Left)).unwrap());
  let tile_ids = match (top, left) {
    (Some(top), Some(left)) => top.intersection(left).copied().collect(),
    (Some(a), None) | (None, Some(a)) => a.clone(),
    (None, None) => HashSet::new(),
  };
  tile_ids.into_iter().filter_map(move |(id_a, id_b)| {
    if !tiles.contains_key(&id_a) {
      return None;
    }
    let mut tiles = tiles.clone();
    let tile = tiles.remove(&id_a).unwrap().remove(id_b);
    let mut board = board.clone();
    board.last_mut().unwrap().push((id_a, tile));
    Some((board, tiles))
  })
}

fn get_all_translations<T>(t0: Vec<Vec<T>>) -> Vec<Vec<Vec<T>>>
where
  T: Copy,
{
  let t1 = rotate_left(&t0);
  let t2 = rotate_left(&t1);
  let t3 = rotate_left(&t2);
  let t4 = transpose(&t0);
  let t5 = rotate_left(&t4);
  let t6 = rotate_left(&t5);
  let t7 = rotate_left(&t6);
  vec![t0, t1, t2, t3, t4, t5, t6, t7]
}

fn parse(s: &str) -> Vec<(u64, Tile)> {
  s.split("\n\n").map(parse_one).collect()
}

static TILE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Tile (\d+):$").unwrap());

fn parse_one(s: &str) -> (u64, Tile) {
  let mut lines = s.split('\n');
  let fst = lines.next().unwrap();
  let fst_caps = TILE.captures(fst).unwrap();
  let num: u64 = fst_caps[1].parse().unwrap();
  let tile: Tile = lines
    .filter(|line| !line.is_empty())
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '#' => Pixel::B,
          '.' => Pixel::W,
          _ => panic!("bad pixel: {}", c),
        })
        .collect()
    })
    .collect();
  (num, tile)
}

#[test]
fn t() {
  let inp = include_str!("input/d20.txt");
  assert_eq!(p1(inp), 12519494280967);
  // assert_eq!(p2(inp), ___);
}
