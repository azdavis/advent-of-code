use helpers::matrix::{neighbors, Coord};
use helpers::HashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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

pub fn p1(s: &str) -> u32 {
  let grid = parse(s);
  grid
    .iter()
    .enumerate()
    .map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .filter_map(|(x, &n1)| {
          neighbors(&grid, [x, y])
            .all(|(&n2, _)| n1 < n2)
            .then(|| u32::from(n1) + 1)
        })
        .sum::<u32>()
    })
    .sum()
}

const TOP_SIZES: usize = 3;
const MAX: u8 = 9;

pub fn p2(s: &str) -> usize {
  let grid = parse(s);
  let mut visited = HashSet::<Coord>::default();
  let mut sizes = BinaryHeap::<Reverse<usize>>::default();
  for (y, row) in grid.iter().enumerate() {
    for (x, &num) in row.iter().enumerate() {
      if visited.contains(&[x, y]) || num == MAX {
        continue;
      }
      let mut size = 0usize;
      let mut stack = vec![[x, y]];
      while let Some([x, y]) = stack.pop() {
        if visited.contains(&[x, y]) || grid[y][x] == MAX {
          continue;
        }
        visited.insert([x, y]);
        size += 1;
        stack.extend(neighbors(&grid, [x, y]).map(|it| it.1));
      }
      sizes.push(Reverse(size));
      if sizes.len() > TOP_SIZES {
        sizes.pop();
      }
    }
  }
  assert_eq!(sizes.len(), TOP_SIZES);
  sizes.into_iter().map(|it| it.0).product()
}

#[test]
fn t() {
  let s = include_str!("input/d09.txt");
  assert_eq!(p1(s), 539);
  assert_eq!(p2(s), 736920);
}
