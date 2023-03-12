use helpers::dijkstra::{dijkstra, Graph};
use helpers::neighbors::{neighbors, Coord};
use helpers::{HashMap, HashSet};
use std::ascii::escape_default;

type Portal = [char; 2];

const PORTAL_LEN: usize = 2;
const START: Portal = ['A', 'A'];
const END: Portal = ['Z', 'Z'];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PortalLoc {
  Outer,
  Inner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
  Wall,
  Blank,
  Corridor,
  Portal(Portal, PortalLoc),
}

type Portals = HashMap<Portal, HashSet<Coord>>;

struct Board {
  tiles: Vec<Vec<Tile>>,
  portals: Portals,
}

struct PortalBoard(Board);

impl Graph for PortalBoard {
  type Node = Coord;

  fn neighbors(&self, [x, y]: Self::Node) -> HashSet<Self::Node> {
    let nearby = neighbors(&self.0.tiles, [x, y])
      .filter_map(|(t, node)| matches!(t, Tile::Corridor | Tile::Portal(_, _)).then(|| node));
    let warp = self.0.tiles.get(y).and_then(|r| match r.get(x)? {
      Tile::Portal(p, _) => {
        let set = self.0.portals.get(p)?;
        set.iter().find(|&&node| node != [x, y]).copied()
      }
      _ => None,
    });
    nearby.chain(warp).collect()
  }
}

struct RecursiveBoard(Board);

impl Graph for RecursiveBoard {
  type Node = [usize; 3];

  fn neighbors(&self, [x, y, z]: Self::Node) -> HashSet<Self::Node> {
    let nearby = neighbors(&self.0.tiles, [x, y]).filter_map(|(t, [x, y])| {
      let ok = match *t {
        Tile::Wall | Tile::Blank => false,
        Tile::Corridor => true,
        Tile::Portal(name, loc) => match loc {
          PortalLoc::Outer => (z == 0) == (name == START || name == END),
          PortalLoc::Inner => true,
        },
      };
      ok.then_some([x, y, z])
    });
    let warp = self.0.tiles.get(y).and_then(|r| match r.get(x)? {
      Tile::Portal(p, loc) => {
        let set = self.0.portals.get(p)?;
        let &[x, y] = set.iter().find(|&&node| node != [x, y])?;
        let z = match loc {
          PortalLoc::Outer => z - 1,
          PortalLoc::Inner => z + 1,
        };
        Some([x, y, z])
      }
      _ => None,
    });
    nearby.chain(warp).collect()
  }
}

fn parse(text: &str) -> (Board, Coord, Coord) {
  let lines: Vec<_> = text.lines().map(str::as_bytes).collect();
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
  let row_len = tiles[PORTAL_LEN].len() + PORTAL_LEN;
  for row in &mut tiles {
    if row.len() < row_len {
      row.resize(row_len, Tile::Blank);
    }
  }
  let mut portals = Portals::default();
  for (y1, &line) in lines.iter().enumerate() {
    for (x1, &b1) in line.iter().enumerate() {
      if !b1.is_ascii_alphabetic() {
        continue;
      }
      let (b2, [x2, y2]) = neighbors(&lines, [x1, y1])
        .find_map(|(&b, xy)| b.is_ascii_alphabetic().then_some((b, xy)))
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
      let loc = if x1.min(x2) == 0
        || x1.max(x2) == row_len - 1
        || y1.min(y2) == 0
        || y1.max(y2) == tiles.len() - 1
      {
        PortalLoc::Outer
      } else {
        PortalLoc::Inner
      };
      tiles[y][x] = Tile::Portal(portal, loc);
      portals.entry(portal).or_default().insert([x, y]);
    }
  }
  let &start = portals[&START].iter().next().unwrap();
  let &end = portals[&END].iter().next().unwrap();
  (Board { tiles, portals }, start, end)
}

pub fn p1(s: &str) -> usize {
  let (board, start, end) = parse(s);
  dijkstra(&PortalBoard(board), start, end).unwrap()
}

pub fn p2(s: &str) -> usize {
  let (board, [sx, sy], [ex, ey]) = parse(s);
  dijkstra(&RecursiveBoard(board), [sx, sy, 0], [ex, ey, 0]).unwrap()
}

#[test]
fn t() {
  let s = include_str!("input/d20.txt");
  assert_eq!(p1(s), 448);
  assert_eq!(p2(s), 5678);
}
