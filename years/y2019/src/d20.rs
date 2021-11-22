use helpers::{HashMap, HashSet};
use std::ascii::escape_default;

type Portal = [char; 2];
type Coord = [usize; 2];

#[derive(Debug, Clone, Copy)]
enum Tile {
  Wall,
  Corridor,
  Blank,
  Portal(Portal),
}

type Portals = HashMap<Portal, HashSet<Coord>>;

struct Board {
  tiles: Vec<Vec<Tile>>,
  portals: Portals,
}

fn neighbors<'a, T>(
  matrix: &'a [&'a [T]],
  x: usize,
  y: usize,
) -> impl Iterator<Item = (&T, Coord)> {
  [
    x.checked_add(1).map(|x| (x, y)),
    x.checked_sub(1).map(|x| (x, y)),
    y.checked_add(1).map(|y| (x, y)),
    y.checked_sub(1).map(|y| (x, y)),
  ]
  .into_iter()
  .filter_map(|xy| {
    let (x, y) = xy?;
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
          b'.' => Tile::Corridor,
          b' ' => Tile::Blank,
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
      let (b2, [x2, y2]) = neighbors(&lines, x1, y1)
        .find_map(|(&b, xy)| b.is_ascii_alphabetic().then(|| (b, xy)))
        .unwrap();
      let (_, [x, y]) = neighbors(&lines, x1, y1)
        .chain(neighbors(&lines, x2, y2))
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
  let &[x, y] = board.portals[&['A', 'A']].iter().next().unwrap();
  assert!(matches!(board.tiles[y][x], Tile::Portal(_)));
  s.len()
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  // let s = include_str!("input/d20.txt");
  // assert_eq!(p1(s), ___);
  // assert_eq!(p2(s), ___);
}
