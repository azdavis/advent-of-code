type Tile = u32;
type Board = Vec<Vec<(Tile, bool)>>;

fn parse(s: &str) -> (Vec<Tile>, Vec<Board>) {
  let mut iter = s.lines();
  let draws: Vec<Tile> = iter
    .next()
    .unwrap()
    .split(',')
    .map(|x| x.parse().unwrap())
    .collect();
  let mut boards = Vec::<Board>::new();
  while let Some(blank) = iter.next() {
    assert!(blank.is_empty());
    let board: Board = iter
      .by_ref()
      .take(5)
      .map(|line| {
        line
          .split_ascii_whitespace()
          .map(|x| (x.parse().unwrap(), false))
          .collect()
      })
      .collect();
    boards.push(board);
  }
  (draws, boards)
}

fn play(draw: Tile, board: &mut Board) {
  for row in board.iter_mut() {
    for (tile, chosen) in row.iter_mut() {
      if *tile == draw {
        *chosen = true;
      }
    }
  }
}

#[allow(clippy::ptr_arg)]
fn is_win(board: &Board) -> bool {
  board.iter().any(|row| row.iter().all(|x| x.1))
    || (0..board.len()).any(|i| board.iter().all(|row| row[i].1))
}

#[allow(clippy::ptr_arg)]
fn score(draw: Tile, board: &Board) -> u32 {
  let s: u32 = board
    .iter()
    .flatten()
    .filter_map(|&(n, chosen)| (!chosen).then(|| n))
    .sum();
  s * draw
}

pub fn p1(s: &str) -> u32 {
  let (draws, mut boards) = parse(s);
  for draw in draws {
    for board in boards.iter_mut() {
      play(draw, board);
      if is_win(board) {
        return score(draw, board);
      }
    }
  }
  panic!("no solution")
}

pub fn p2(s: &str) -> u32 {
  let (draws, mut boards) = parse(s);
  for draw in draws {
    let n_boards = boards.len();
    let mut new_boards = Vec::with_capacity(n_boards);
    for mut board in boards {
      play(draw, &mut board);
      if is_win(&board) {
        if n_boards == 1 {
          return score(draw, &board);
        }
        // else, discard this board
      } else {
        new_boards.push(board);
      }
    }
    boards = new_boards;
  }
  panic!("no solution")
}

#[test]
fn t() {
  let s = include_str!("input/d04.txt");
  assert_eq!(p1(s), 38913);
  assert_eq!(p2(s), 16836);
}
