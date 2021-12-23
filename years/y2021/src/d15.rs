use helpers::dijkstra::{dijkstra, Graph};
use helpers::neighbors::neighbors;

fn parse(s: &str) -> Vec<Vec<u8>> {
  s.lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect()
    })
    .collect()
}

fn run(matrix: Vec<Vec<u8>>) -> usize {
  let end = [matrix.last().unwrap().len() - 1, matrix.len() - 1];
  dijkstra(&RiskGraph { matrix }, [0, 0], end).unwrap()
}

struct RiskGraph {
  matrix: Vec<Vec<u8>>,
}

impl Graph for RiskGraph {
  type Node = [usize; 2];

  fn neighbors(&self, node: Self::Node) -> helpers::HashSet<Self::Node> {
    neighbors(&self.matrix, node).map(|it| it.1).collect()
  }

  fn distance(&self, _: Self::Node, b: Self::Node) -> usize {
    let [x, y] = b;
    self.matrix[y][x].into()
  }
}

pub fn p1(s: &str) -> usize {
  run(parse(s))
}

const REPEAT: usize = 5;

pub fn p2(s: &str) -> usize {
  let tile = parse(s);
  let tile_w = tile.first().unwrap().len();
  let tile_h = tile.len();
  let mut matrix = Vec::<Vec<u8>>::with_capacity(tile_h * REPEAT);
  for y in 0..REPEAT {
    let y_start = y * tile_h;
    let y_end = (y + 1) * tile_h;
    for _ in 0..tile_h {
      matrix.push(Vec::with_capacity(tile_w * REPEAT));
    }
    for x in 0..REPEAT {
      let mut tile = tile.clone();
      let add: u8 = (x + y).try_into().unwrap();
      for row in tile.iter_mut() {
        for x in row.iter_mut() {
          *x += add;
          while *x > 9 {
            *x -= 9;
          }
        }
      }
      for (ext, row) in tile.into_iter().zip(&mut matrix[y_start..y_end]) {
        row.extend(ext);
      }
    }
  }
  run(matrix)
}

#[test]
fn t() {
  let s = include_str!("input/d15.txt");
  assert_eq!(p1(s), 456);
  assert_eq!(p2(s), 2831);
}

#[test]
fn ex1() {
  let s = include_str!("input/d15_ex1.txt");
  assert_eq!(p1(s), 40);
  assert_eq!(p2(s), 315);
}
