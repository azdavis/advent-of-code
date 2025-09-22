#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
  Down,
  Right,
}

type Coord = [usize; 2];

fn go(state: &mut [Vec<Option<Dir>>], want: Dir, f: fn(Coord, Coord) -> Coord) {
  let w = state.first().unwrap().len();
  let h = state.len();
  let mut to_move = Vec::new();
  for (row_idx, row) in state.iter().enumerate() {
    for (col_idx, cuke) in row.iter().enumerate() {
      if *cuke != Some(want) {
        continue;
      }
      let [new_row, new_col] = f([row_idx, col_idx], [w, h]);
      if state[new_row][new_col].is_none() {
        to_move.push([row_idx, col_idx]);
      }
    }
  }
  for [row_idx, col_idx] in to_move {
    let [new_row, new_col] = f([row_idx, col_idx], [w, h]);
    state[row_idx][col_idx] = None;
    state[new_row][new_col] = Some(want);
  }
}

pub fn p1(s: &str) -> usize {
  let mut state: Vec<Vec<_>> = s
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '.' => None,
          'v' => Some(Dir::Down),
          '>' => Some(Dir::Right),
          _ => panic!("unknown char: {c}"),
        })
        .collect()
    })
    .collect();
  let mut step = 0usize;
  loop {
    let old_state = state.clone();
    go(&mut state, Dir::Right, |[r, c], [w, _]| {
      let c = if c + 1 == w { 0 } else { c + 1 };
      [r, c]
    });
    go(&mut state, Dir::Down, |[r, c], [_, h]| {
      let r = if r + 1 == h { 0 } else { r + 1 };
      [r, c]
    });
    step += 1;
    if state == old_state {
      return step;
    }
  }
}

#[test]
fn t() {
  let s = include_str!("input/d25.txt");
  assert_eq!(p1(s), 353);
}

#[test]
fn ex1() {
  let s = include_str!("input/d25_ex1.txt");
  assert_eq!(p1(s), 58);
}
