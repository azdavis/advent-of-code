pub fn p1(s: &str) -> usize {
  let mut prev = parse(s);
  loop {
    let cur = evolve(&prev);
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

type Grid = Vec<Vec<Tile>>;

#[allow(clippy::needless_range_loop)]
#[allow(clippy::ptr_arg)]
fn evolve(xs: &Grid) -> Grid {
  let mut ret = xs.clone();
  for i in 0..ret.len() {
    for j in 0..ret[i].len() {
      match ret[i][j] {
        Tile::Floor => {}
        Tile::Empty => {
          let any_occupied = adjacent(i, j)
            .into_iter()
            .filter_map(|(i, j)| xs.get(i)?.get(j))
            .any(|&x| matches!(x, Tile::Occupied));
          if !any_occupied {
            ret[i][j] = Tile::Occupied;
          }
        }
        Tile::Occupied => {
          let count = adjacent(i, j)
            .into_iter()
            .filter_map(|(i, j)| xs.get(i)?.get(j))
            .filter(|&x| matches!(x, Tile::Occupied))
            .count();
          if count >= 4 {
            ret[i][j] = Tile::Empty;
          }
        }
      }
    }
  }
  ret
}

// pretty ugly
fn adjacent(i: usize, j: usize) -> Vec<(usize, usize)> {
  let mut ret = vec![(i + 1, j + 1), (i + 1, j), (i, j + 1)];
  if let Some(i_sub_1) = i.checked_sub(1) {
    ret.push((i_sub_1, j + 1));
    ret.push((i_sub_1, j));
  }
  if let Some(j_sub_1) = j.checked_sub(1) {
    ret.push((i + 1, j_sub_1));
    ret.push((i, j_sub_1));
  }
  if let Some(i_sub_1) = i.checked_sub(1) {
    if let Some(j_sub_1) = j.checked_sub(1) {
      ret.push((i_sub_1, j_sub_1));
    }
  }
  ret
}

fn parse(s: &str) -> Grid {
  s.split('\n')
    .filter(|line| !line.is_empty())
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
