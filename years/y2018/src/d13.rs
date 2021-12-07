use helpers::matrix::Coord;
use helpers::{Compass, HashMap, HashSet};

enum Tile {
  /// Shown as `/`
  TurnUp,
  /// Shown as `\`
  TurnDown,
  /// Shown as `+`
  Intersection,
}

enum Turn {
  Left,
  Straight,
  Right,
}

type Tiles = HashMap<Coord, Tile>;
type Carts = HashMap<Coord, (Turn, Compass)>;

fn ignore<T>(_: T) {}

fn parse(s: &str) -> (Tiles, Carts) {
  let mut tiles = Tiles::default();
  let mut carts = Carts::default();
  for (y, line) in s.lines().enumerate() {
    for (x, c) in line.chars().enumerate() {
      let pos = [x, y];
      match c {
        '/' => ignore(tiles.insert(pos, Tile::TurnUp)),
        '\\' => ignore(tiles.insert(pos, Tile::TurnDown)),
        '+' => ignore(tiles.insert(pos, Tile::Intersection)),
        '^' => ignore(carts.insert(pos, (Turn::Left, Compass::North))),
        'v' => ignore(carts.insert(pos, (Turn::Left, Compass::South))),
        '<' => ignore(carts.insert(pos, (Turn::Left, Compass::West))),
        '>' => ignore(carts.insert(pos, (Turn::Left, Compass::East))),
        '|' | '-' | ' ' => {}
        _ => panic!("unknown char: {}", c),
      }
    }
  }
  (tiles, carts)
}

fn evolve_cart(tiles: &Tiles, pos: Coord, carts: &mut Carts) -> Option<Coord> {
  let (mut turn, mut facing) = carts.remove(&pos).unwrap();
  let [x, y] = pos;
  let pos = match facing {
    Compass::North => [x, y - 1],
    Compass::South => [x, y + 1],
    Compass::East => [x + 1, y],
    Compass::West => [x - 1, y],
  };
  match tiles.get(&pos) {
    None => {}
    Some(&Tile::TurnUp) => {
      facing = match facing {
        Compass::North => Compass::East,
        Compass::South => Compass::West,
        Compass::East => Compass::North,
        Compass::West => Compass::South,
      };
    }
    Some(&Tile::TurnDown) => {
      facing = match facing {
        Compass::North => Compass::West,
        Compass::South => Compass::East,
        Compass::East => Compass::South,
        Compass::West => Compass::North,
      };
    }
    Some(&Tile::Intersection) => {
      match turn {
        Turn::Left => facing = facing.left(),
        Turn::Straight => {}
        Turn::Right => facing = facing.right(),
      }
      turn = match turn {
        Turn::Left => Turn::Straight,
        Turn::Straight => Turn::Right,
        Turn::Right => Turn::Left,
      };
    }
  }
  carts.insert(pos, (turn, facing)).map(|_| pos)
}

fn mk_order(carts: &Carts) -> Vec<Coord> {
  let mut ret: Vec<_> = carts.keys().copied().collect();
  ret.sort_unstable_by_key(|&[x, y]| (y, x));
  ret
}

pub fn p1(s: &str) -> Coord {
  let (tiles, mut carts) = parse(s);
  loop {
    for old_pos in mk_order(&carts) {
      if let Some(new_pos) = evolve_cart(&tiles, old_pos, &mut carts) {
        return new_pos;
      }
    }
  }
}

pub fn p2(s: &str) -> Coord {
  let (tiles, mut carts) = parse(s);
  loop {
    if carts.len() == 1 {
      return carts.keys().copied().next().unwrap();
    }
    let mut dead = HashSet::<Coord>::default();
    for old_pos in mk_order(&carts) {
      if dead.contains(&old_pos) {
        continue;
      }
      if let Some(new_pos) = evolve_cart(&tiles, old_pos, &mut carts) {
        dead.insert(new_pos);
      }
    }
    for pos in dead {
      carts.remove(&pos);
    }
  }
}

#[test]
fn t() {
  let s = include_str!("input/d13.txt");
  assert_eq!(p1(s), [124, 130]);
  assert_eq!(p2(s), [143, 123]);
}

#[test]
fn ex1() {
  let s = include_str!("input/d13_ex1.txt");
  assert_eq!(p1(s), [7, 3]);
}

#[test]
fn ex2() {
  let s = include_str!("input/d13_ex2.txt");
  assert_eq!(p2(s), [6, 4]);
}
