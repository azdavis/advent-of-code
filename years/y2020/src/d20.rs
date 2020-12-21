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

pub fn p2(s: &str) -> usize {
  let mut board = go(s);
  // delete the edges of each tile
  for row in board.iter_mut() {
    for (_, tile) in row.iter_mut() {
      tile.pop().unwrap();
      tile.remove(0);
      for tile_row in tile.iter_mut() {
        tile_row.pop().unwrap();
        tile_row.remove(0);
      }
    }
  }
  // merge the tiles together into one big tile
  let tile_dim = board.first().unwrap().first().unwrap().1.len();
  let constructed: Tile = board
    .into_iter()
    .flat_map(|mut row| {
      let new_rows: Vec<_> = (0..tile_dim)
        .map(|_| {
          row
            .iter_mut()
            .flat_map(|(_, tile)| tile.pop().unwrap())
            .collect()
        })
        .collect();
      new_rows.into_iter().rev()
    })
    .collect();
  // collect the set of sea monster points
  let sea_monster: HashSet<_> = include_str!("input/d20_sea_monster.txt")
    .split('\n')
    .filter(|line| !line.is_empty())
    .rev()
    .enumerate()
    .flat_map(|(y, line)| {
      line.chars().enumerate().filter_map(move |(x, c)| match c {
        '#' => Some((y, x)),
        ' ' => None,
        _ => panic!("bad char: {}", c),
      })
    })
    .collect();
  // consider all translations of the big board
  for board in get_all_translations(constructed) {
    // consider each position on the board; if, starting from that position, it
    // is a sea monster, then note all sea monster points. collect all such
    // points into a set
    let deleted: HashSet<_> = board
      .iter()
      .enumerate()
      .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (y, x)))
      .filter_map(|(y, x)| {
        let all_hit = sea_monster.iter().all(|&(sm_y, sm_x)| {
          board
            .get(y + sm_y)
            .and_then(|row| row.get(x + sm_x))
            .map(|&px| matches!(px, Pixel::B))
            .unwrap_or_default()
        });
        if all_hit {
          let ret = sea_monster
            .iter()
            .map(move |&(sm_y, sm_x)| (sm_y + y, sm_x + x));
          Some(ret)
        } else {
          None
        }
      })
      .flatten()
      .collect();
    // if we found at least one sea monster, this orientation is the one.
    if !deleted.is_empty() {
      let black_count = board
        .iter()
        .flat_map(|row| row.iter().filter(|&px| matches!(px, Pixel::B)))
        .count();
      return black_count - deleted.len();
    }
  }
  panic!("no solution")
}

/// a 2d array of pixels
type Tile = Vec<Vec<Pixel>>;

/// a map from tile ID to the translations of each tile
type Tiles = HashMap<u64, Vec<Tile>>;

/// a 2d array of (tile id, tile)
type Board = Vec<Vec<(u64, Tile)>>;

/// a map from (sequence of pixels, direction) to a set of (tile id, translation
/// index).
///
/// we use this to answer the question 'what tiles, translated in what way, have
/// an edge, facing in a given direction, equal to the given sequence of
/// pixels'?
///
/// this can be derived from a `Tiles`.
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

fn go(s: &str) -> Board {
  let tiles = parse(s);
  // there must be a square number of tiles
  let n = sqrt(tiles.len());
  // for each tile, get all of its translations
  let tiles: Tiles = tiles
    .into_iter()
    .map(|(n, t0)| (n, get_all_translations(t0)))
    .collect();
  let edges = mk_edges(&tiles);
  // try each tile as the starting (upper-left corner) tile
  for &id_a in tiles.keys() {
    // try each translation of the tile
    let mut tiles = tiles.clone();
    let mut candidates: Vec<(Board, Tiles)> = tiles
      .remove(&id_a)
      .unwrap()
      .into_iter()
      .map(|tile| (vec![vec![(id_a, tile)]], tiles.clone()))
      .collect();
    // try building the entire board, tile by tile
    for row in 0..n {
      // need to add an empty row each time we start a new row, except the first
      // time
      if row != 0 {
        for (board, _) in candidates.iter_mut() {
          board.push(vec![]);
        }
      }
      for col in 0..n {
        // already put in the starting tile
        if row == 0 && col == 0 {
          continue;
        }
        candidates = candidates
          .into_iter()
          .flat_map(|(board, tiles)| expand(&edges, board, tiles))
          .collect();
      }
    }
    if let Some((board, _)) = candidates.pop() {
      // candidates might not be empty; it might contain translations of `board`
      return board;
    }
  }
  panic!("no solution")
}

fn mk_edges(tiles: &Tiles) -> Edges {
  let mut ret = Edges::new();
  for (&id_a, tile_translations) in tiles.iter() {
    for (id_b, tile) in tile_translations.iter().enumerate() {
      for &(f, dir) in FNS.iter() {
        ret.entry((f(&tile), dir)).or_default().insert((id_a, id_b));
      }
    }
  }
  ret
}

#[allow(clippy::type_complexity)]
const FNS: [(for<'a> fn(&'a [Vec<Pixel>]) -> Vec<Pixel>, Dir); 4] = [
  (top, Dir::Top),
  (bot, Dir::Bot),
  (left, Dir::Left),
  (right, Dir::Right),
];

/// returns the exact square root of `n` if there is one.
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

/// given a `board` under construction, a set of unused `tiles`, and the `Edges`
/// from the original set of tiles, returns an iterator of the possible pairs of
/// (new board, remaining unused tiles).
fn expand(
  edges: &Edges,
  board: Board,
  tiles: Tiles,
) -> impl Iterator<Item = (Board, Tiles)> {
  let row = board.len() - 1;
  let col = board[row].len();
  // get the sets of possible tile IDs, based on the restrictions from the tile
  // already above and to the left of the location we're trying to put a tile.
  let top = row
    .checked_sub(1)
    .map(|row| edges.get(&(bot(&board[row][col].1), Dir::Top)).unwrap());
  let left = col
    .checked_sub(1)
    .map(|col| edges.get(&(right(&board[row][col].1), Dir::Left)).unwrap());
  // combine these to get the overall set of possible tile IDs.
  let tile_ids = match (top, left) {
    // if there were restrictions from both the top and left, the valid IDs to
    // try must be in both sets.
    (Some(top), Some(left)) => top.intersection(left).copied().collect(),
    (Some(a), None) | (None, Some(a)) => a.clone(),
    (None, None) => HashSet::new(),
  };
  tile_ids.into_iter().filter_map(move |(id_a, id_b)| {
    // if we've already used a tile, we can't use it again.
    if !tiles.contains_key(&id_a) {
      return None;
    }
    // remove that tile and add it to the board with the given translation.
    let mut tiles = tiles.clone();
    let tile = tiles.remove(&id_a).unwrap().remove(id_b);
    let mut board = board.clone();
    board.last_mut().unwrap().push((id_a, tile));
    Some((board, tiles))
  })
}

/// returns all translations of the matrix possible by flipping and rotating.
/// the return value will always have length 8, but it's still annoying to work
/// with arrays in many ways, so we use a vec.
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
  assert_eq!(p2(inp), 2442);
}
