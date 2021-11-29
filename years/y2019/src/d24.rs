use helpers::matrix::neighbors;
use helpers::{hash_map, hash_set};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
  Bug,
  Empty,
}

impl Tile {
  fn is_bug(&self) -> bool {
    matches!(*self, Tile::Bug)
  }
}

fn parse(s: &str) -> Vec<Vec<Tile>> {
  s.lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '#' => Tile::Bug,
          '.' | '?' => Tile::Empty,
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

fn new_tile(tile: Tile, bugs: usize) -> Tile {
  match (tile, bugs) {
    (Tile::Bug, 1) | (Tile::Empty, 1 | 2) => Tile::Bug,
    _ => Tile::Empty,
  }
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
          .map(|(x, &tile)| {
            let bugs = neighbors(&cur, [x, y])
              .filter(|&(&tile, _)| tile.is_bug())
              .count();
            new_tile(tile, bugs)
          })
          .collect()
      })
      .collect();
    if !past.insert(cur.clone()) {
      return biodiversity_rating(&cur);
    }
  }
}

type Coord3 = (isize, usize, usize);

fn neighbors3(coord: Coord3, n: usize) -> Vec<Coord3> {
  let mid = n / 2;
  let (level, x, y) = coord;
  let mut ret = Vec::with_capacity(4);
  // left
  if x == 0 {
    ret.push((level - 1, mid - 1, mid));
  } else if x == mid + 1 && y == mid {
    ret.extend((0..n).map(|y| (level + 1, n - 1, y)));
  } else {
    ret.push((level, x - 1, y));
  }
  // top
  if y == 0 {
    ret.push((level - 1, mid, mid - 1));
  } else if y == mid + 1 && x == mid {
    ret.extend((0..n).map(|x| (level + 1, x, n - 1)));
  } else {
    ret.push((level, x, y - 1));
  }
  // right
  if x == n - 1 {
    ret.push((level - 1, mid + 1, mid));
  } else if x == mid - 1 && y == mid {
    ret.extend((0..n).map(|y| (level + 1, 0, y)));
  } else {
    ret.push((level, x + 1, y));
  }
  // bot
  if y == n - 1 {
    ret.push((level - 1, mid, mid + 1));
  } else if y == mid - 1 && x == mid {
    ret.extend((0..n).map(|x| (level + 1, x, 0)));
  } else {
    ret.push((level, x, y + 1));
  }
  ret
}

fn run3(s: &str, rounds: usize) -> usize {
  let init = parse(s);
  let n = init.len();
  assert_eq!(n % 2, 1);
  let mid = n / 2;
  let mut cur = hash_map([(0, init)]);
  for _ in 0..rounds {
    let min = cur.keys().copied().min().unwrap();
    let max = cur.keys().copied().max().unwrap();
    assert_eq!(cur.len(), usize::try_from(max - min + 1).unwrap());
    if cur[&min].iter().flatten().any(Tile::is_bug) {
      cur.insert(min - 1, vec![vec![Tile::Empty; n]; n]);
    }
    if cur[&max].iter().flatten().any(Tile::is_bug) {
      cur.insert(max + 1, vec![vec![Tile::Empty; n]; n]);
    }
    cur = cur
      .iter()
      .map(|(&level, grid)| {
        let grid: Vec<_> = grid
          .iter()
          .enumerate()
          .map(|(y, row)| {
            row
              .iter()
              .enumerate()
              .map(|(x, &tile)| {
                if x == mid && y == mid {
                  // the middle tile doesn't actually exist
                  return Tile::Empty;
                }
                let bugs = neighbors3((level, x, y), n)
                  .into_iter()
                  .filter(|&(level, x, y)| {
                    cur
                      .get(&level)
                      .map_or(Tile::Empty, |grid| grid[y][x])
                      .is_bug()
                  })
                  .count();
                new_tile(tile, bugs)
              })
              .collect()
          })
          .collect();
        (level, grid)
      })
      .collect()
  }
  cur
    .values()
    .flatten()
    .flatten()
    .filter(|&tile| tile.is_bug())
    .count()
}

pub fn p2(s: &str) -> usize {
  run3(s, 200)
}

#[test]
fn t() {
  let s = include_str!("input/d24.txt");
  assert_eq!(p1(s), 28903899);
  assert_eq!(p2(s), 1896);
}

#[cfg(test)]
mod examples {
  use crate::d24::run3;

  use super::{biodiversity_rating, neighbors3, parse, Coord3};

  #[test]
  fn t_biodiversity_rating() {
    let s = r#"
.....
.....
.....
#....
.#...
"#;
    assert_eq!(biodiversity_rating(&parse(s.trim())), 2129920);
  }

  #[test]
  fn t_p2_small() {
    let s = r#"
....#
#..#.
#.?##
..#..
#....
"#;
    assert_eq!(run3(s.trim(), 10), 99);
  }

  /// since I ended up writing every possible fn call for `neighbors3` with `n =
  /// 5` as a test here, the function itself could just be implemented as a
  /// table lookup. _shrug_
  #[rustfmt::skip]
  mod every_neighbors3_n5 {
    use super::{neighbors3, Coord3};

    /*
    |-----+-----+---------+-----+-----|
    |     |     |         |     |     |
    |  A  |  B  |    C    |  D  |  E  |
    |     |     |         |     |     |
    |-----+-----+---------+-----+-----|
    |     |     |         |     |     |
    |  F  |  G  |    H    |  I  |  J  |
    |     |     |         |     |     |
    |-----+-----+---------+-----+-----|
    |     |     |A|B|C|D|E|     |     |
    |     |     |-+-+-+-+-|     |     |
    |     |     |F|G|H|I|J|     |     |
    |     |     |-+-+-+-+-|     |     |
    |  K  |  L  |K|L|?|N|O|  N  |  O  |
    |     |     |-+-+-+-+-|     |     |
    |     |     |P|Q|R|S|T|     |     |
    |     |     |-+-+-+-+-|     |     |
    |     |     |U|V|W|X|Y|     |     |
    |-----+-----+---------+-----+-----|
    |     |     |         |     |     |
    |  P  |  Q  |    R    |  S  |  T  |
    |     |     |         |     |     |
    |-----+-----+---------+-----+-----|
    |     |     |         |     |     |
    |  U  |  V  |    W    |  X  |  Y  |
    |     |     |         |     |     |
    |-----+-----+---------+-----+-----|
     */

    type Coord2 = (usize, usize);

    macro_rules! p {
      ($name: ident, $x: expr, $y: expr) => {
        const $name: Coord2 = ($x, $y);
      };
    }

    p!{A, 0, 0} p!{B, 1, 0} p!{C, 2, 0} p!{D, 3, 0} p!{E, 4, 0}
    p!{F, 0, 1} p!{G, 1, 1} p!{H, 2, 1} p!{I, 3, 1} p!{J, 4, 1}
    p!{K, 0, 2} p!{L, 1, 2}             p!{N, 3, 2} p!{O, 4, 2}
    p!{P, 0, 3} p!{Q, 1, 3} p!{R, 2, 3} p!{S, 3, 3} p!{T, 4, 3}
    p!{U, 0, 4} p!{V, 1, 4} p!{W, 2, 4} p!{X, 3, 4} p!{Y, 4, 4}

    fn ck<const LEN: usize>(coord: Coord3, want: [Coord3; LEN]) {
      let mut want = want.to_vec();
      want.sort_unstable();
      let mut got = neighbors3(coord, 5);
      got.sort_unstable();
      assert_eq!(want, got);
    }

    fn d((x, y): Coord2) -> Coord3 { (-1, x, y) }
    fn z((x, y): Coord2) -> Coord3 { (0,  x, y) }
    fn u((x, y): Coord2) -> Coord3 { (1,  x, y) }

    #[test] fn ta() { ck(z(A), [z(B), z(F), d(H), d(L)]); }
    #[test] fn tb() { ck(z(B), [z(A), z(C), z(G), d(H)]); }
    #[test] fn tc() { ck(z(C), [z(B), z(D), z(H), d(H)]); }
    #[test] fn td() { ck(z(D), [z(C), z(E), z(I), d(H)]); }
    #[test] fn te() { ck(z(E), [z(D), z(J), d(H), d(N)]); }
    #[test] fn tf() { ck(z(F), [z(A), z(G), z(K), d(L)]); }
    #[test] fn tg() { ck(z(G), [z(B), z(F), z(H), z(L)]); }
    #[test] fn th() { ck(z(H), [z(C), z(G), z(I), u(A), u(B), u(C), u(D), u(E)]); }
    #[test] fn ti() { ck(z(I), [z(D), z(H), z(J), z(N)]); }
    #[test] fn tj() { ck(z(J), [z(E), z(I), z(O), d(N)]); }
    #[test] fn tk() { ck(z(K), [z(F), z(L), z(P), d(L)]); }
    #[test] fn tl() { ck(z(L), [z(G), z(K), z(Q), u(A), u(F), u(K), u(P), u(U)]); }
    #[test] fn tn() { ck(z(N), [z(I), z(O), z(S), u(E), u(J), u(O), u(T), u(Y)]); }
    #[test] fn to() { ck(z(O), [z(J), z(N), z(T), d(N)]); }
    #[test] fn tp() { ck(z(P), [z(K), z(Q), z(U), d(L)]); }
    #[test] fn tq() { ck(z(Q), [z(L), z(R), z(V), z(P)]); }
    #[test] fn tr() { ck(z(R), [z(Q), z(S), z(W), u(U), u(V), u(W), u(X), u(Y)]); }
    #[test] fn ts() { ck(z(S), [z(N), z(T), z(R), z(X)]); }
    #[test] fn tt() { ck(z(T), [z(O), z(S), z(Y), d(N)]); }
    #[test] fn tu() { ck(z(U), [z(P), z(V), d(L), d(R)]); }
    #[test] fn tv() { ck(z(V), [z(Q), z(U), z(W), d(R)]); }
    #[test] fn tw() { ck(z(W), [z(V), z(X), z(R), d(R)]); }
    #[test] fn tx() { ck(z(X), [z(S), z(W), z(Y), d(R)]); }
    #[test] fn ty() { ck(z(Y), [z(T), z(X), d(N), d(R)]); }
  }
}
