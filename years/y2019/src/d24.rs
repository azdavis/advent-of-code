use helpers::hash_set;
use helpers::matrix::neighbors;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
  Bug,
  Empty,
}

fn parse(s: &str) -> Vec<Vec<Tile>> {
  s.lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '#' => Tile::Bug,
          '.' => Tile::Empty,
          _ => panic!("unknown char: {}", c),
        })
        .collect()
    })
    .collect()
}

fn biodiversity_rating(xs: &[Vec<Tile>]) -> usize {
  xs.iter()
    .flatten()
    .enumerate()
    .map(|(idx, tile)| match tile {
      Tile::Bug => 1 << idx,
      Tile::Empty => 0,
    })
    .sum()
}

pub fn p1(s: &str) -> usize {
  let mut cur = parse(s);
  let mut past = hash_set([cur.clone()]);
  loop {
    cur = cur
      .iter()
      .enumerate()
      .map(|(y, row)| {
        row
          .iter()
          .enumerate()
          .map(|(x, tile)| {
            let bugs = neighbors(&cur, [x, y])
              .filter(|&(&tile, _)| tile == Tile::Bug)
              .count();
            match (tile, bugs) {
              (Tile::Bug, 1) => Tile::Bug,
              (Tile::Bug, _) => Tile::Empty,
              (Tile::Empty, 1 | 2) => Tile::Bug,
              (Tile::Empty, _) => Tile::Empty,
            }
          })
          .collect()
      })
      .collect();
    if !past.insert(cur.clone()) {
      return biodiversity_rating(&cur);
    }
  }
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  let s = include_str!("input/d24.txt");
  assert_eq!(p1(s), 28903899);
  // assert_eq!(p2(s), ___);
}

#[test]
fn ex1() {
  let b = r#"
.....
.....
.....
#....
.#...
"#;
  assert_eq!(biodiversity_rating(&parse(b.trim())), 2129920);
}
