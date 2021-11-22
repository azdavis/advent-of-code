use helpers::dijkstra::{dijkstra, Graph};
use helpers::{HashMap, HashSet};
use std::ascii::escape_default;
use std::ops::Deref;

type Portal = [char; 2];
type Coord = [usize; 2];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
  Wall,
  Blank,
  Corridor,
  Portal(Portal),
}

impl Tile {
  fn is_walkable(&self) -> bool {
    matches!(self, Tile::Corridor | Tile::Portal(_))
  }
}

type Portals = HashMap<Portal, HashSet<Coord>>;

struct Board {
  tiles: Vec<Vec<Tile>>,
  portals: Portals,
}

impl Graph for Board {
  type Node = Coord;

  fn nodes(&self) -> HashSet<Self::Node> {
    self
      .tiles
      .iter()
      .enumerate()
      .flat_map(|(y, row)| {
        row
          .iter()
          .enumerate()
          .filter_map(move |(x, &t)| t.is_walkable().then(|| [x, y]))
      })
      .collect()
  }

  fn neighbors(&self, [x, y]: Self::Node) -> HashSet<Self::Node> {
    let mut ret: HashSet<_> = neighbors(&self.tiles, [x, y])
      .filter_map(|(t, node)| t.is_walkable().then(|| node))
      .collect();
    if let Some(&Tile::Portal(p)) = self.tiles.get(y).and_then(|r| r.get(x)) {
      ret.extend(
        self
          .portals
          .get(&p)
          .into_iter()
          .flatten()
          .find(|&&node| node != [x, y]),
      );
    }
    ret
  }
}

fn neighbors<'a, M, R, T>(
  matrix: &'a M,
  [x, y]: Coord,
) -> impl Iterator<Item = (&'a T, Coord)>
where
  M: Deref<Target = [R]>,
  R: 'a + Deref<Target = [T]>,
  T: 'a,
{
  [
    x.checked_add(1).map(|x| [x, y]),
    x.checked_sub(1).map(|x| [x, y]),
    y.checked_add(1).map(|y| [x, y]),
    y.checked_sub(1).map(|y| [x, y]),
  ]
  .into_iter()
  .filter_map(|xy| {
    let [x, y] = xy?;
    let v = matrix.get(y)?.get(x)?;
    Some((v, [x, y]))
  })
}

fn parse(s: &str) -> Board {
  let lines: Vec<_> = s.lines().map(|line| line.as_bytes()).collect();
  let mut tiles: Vec<Vec<_>> = lines
    .iter()
    .map(|&line| {
      line
        .iter()
        .map(|&b| match b {
          b'#' => Tile::Wall,
          b' ' => Tile::Blank,
          b'.' => Tile::Corridor,
          _ => {
            if b.is_ascii_alphabetic() {
              // handle this later
              Tile::Blank
            } else {
              panic!("unknown char: {}", escape_default(b))
            }
          }
        })
        .collect()
    })
    .collect();
  let n = tiles.len();
  for row in tiles.iter_mut() {
    if row.len() < n {
      row.resize(n, Tile::Blank);
    }
  }
  let mut portals = Portals::default();
  for (y1, &line) in lines.iter().enumerate() {
    for (x1, &b1) in line.iter().enumerate() {
      if !b1.is_ascii_alphabetic() {
        continue;
      }
      let (b2, [x2, y2]) = neighbors(&lines, [x1, y1])
        .find_map(|(&b, xy)| b.is_ascii_alphabetic().then(|| (b, xy)))
        .unwrap();
      let (_, [x, y]) = neighbors(&lines, [x1, y1])
        .chain(neighbors(&lines, [x2, y2]))
        .find(|&(&b, _)| b == b'.')
        .unwrap();
      let (b1, b2) = if x1 > x2 || y1 > y2 {
        (b2, b1)
      } else {
        (b1, b2)
      };
      let portal = [b1 as char, b2 as char];
      tiles[y][x] = Tile::Portal(portal);
      portals.entry(portal).or_default().insert([x, y]);
    }
  }
  Board { tiles, portals }
}

pub fn p1(s: &str) -> usize {
  let board = parse(s);
  let &start = board.portals[&['A', 'A']].iter().next().unwrap();
  let &end = board.portals[&['Z', 'Z']].iter().next().unwrap();
  dijkstra(&board, start, end).unwrap()
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  let s = include_str!("input/d20.txt");
  assert_eq!(p1(s), 448);
  // assert_eq!(p2(s), ___);
}
