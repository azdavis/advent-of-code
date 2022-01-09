#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Letter {
  A,
  B,
  C,
  D,
}

impl Letter {
  fn from_char(c: char) -> Option<Self> {
    let ret = match c {
      'A' => Self::A,
      'B' => Self::B,
      'C' => Self::C,
      'D' => Self::D,
      _ => return None,
    };
    Some(ret)
  }

  fn energy(&self) -> usize {
    match self {
      Self::A => 1,
      Self::B => 10,
      Self::C => 100,
      Self::D => 1000,
    }
  }
}

type Loc = [usize; 2];

#[derive(Debug, Clone)]
struct State {
  map: Vec<Vec<Option<Letter>>>,
}

impl State {
  fn desired_loc(&self, loc: Loc) -> Loc {
    let letter = self.map[loc[0]][loc[1]].unwrap();
    let want_col = letter_to_col(letter);
    let mut bottom_row = self.map.len() - 1;
    loop {
      let want = [bottom_row, want_col];
      if want == loc {
        return want;
      }
      if let Some(this_letter) = self.map[bottom_row][want_col] {
        if letter == this_letter {
          bottom_row -= 1;
          continue;
        }
      }
      return want;
    }
  }

  fn try_move(&self, cur: Loc, dst: Loc) -> Option<(Self, usize)> {
    let mut steps = 0usize;
    let [mut cur_row, mut cur_col] = cur;
    assert!(self.map[cur_row][cur_col].is_some());
    let [dst_row, dst_col] = dst;
    loop {
      if cur_col == dst_col {
        if cur_row == dst_row {
          break;
        }
        cur_row += 1;
      } else if cur_row == HALLWAY_ROW {
        if cur_col > dst_col {
          cur_col -= 1;
        } else {
          cur_col += 1;
        }
      } else {
        cur_row -= 1;
      }
      if self.map[cur_row][cur_col].is_some() {
        return None;
      }
      steps += 1;
    }
    let mut ret = self.clone();
    let letter = ret.map[cur[0]][cur[1]].take().unwrap();
    ret.map[dst_row][dst_col] = Some(letter);
    Some((ret, steps * letter.energy()))
  }

  fn all_locs(&self) -> impl Iterator<Item = Loc> + '_ {
    self.map.iter().enumerate().flat_map(|(row_idx, row)| {
      row
        .iter()
        .enumerate()
        .filter_map(move |(col, letter)| letter.map(|_| [row_idx, col]))
    })
  }
}

fn letter_to_col(letter: Letter) -> usize {
  match letter {
    Letter::A => 3,
    Letter::B => 5,
    Letter::C => 7,
    Letter::D => 9,
  }
}

const HALLWAY_ROW: usize = 1;
const HALLWAY_COLS: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];

fn run(state: State) -> usize {
  let mut ret = None::<usize>;
  let mut cur = vec![(state, 0usize)];
  while let Some((state, energy)) = cur.pop() {
    let mut done = true;
    for loc in state.all_locs() {
      let desired = state.desired_loc(loc);
      if loc == desired {
        continue;
      }
      done = false;
      if let Some((new_state, new_energy)) = state.try_move(loc, desired) {
        cur.push((new_state, energy + new_energy));
        continue;
      }
      if loc[0] == HALLWAY_ROW {
        continue;
      }
      for col in HALLWAY_COLS {
        let new_pos = [HALLWAY_ROW, col];
        if let Some((new_state, new_energy)) = state.try_move(loc, new_pos) {
          cur.push((new_state, energy + new_energy));
        }
      }
    }
    if done {
      ret = Some(match ret {
        Some(x) => energy.min(x),
        None => energy,
      });
    }
  }
  ret.unwrap()
}

fn parse(s: &str) -> State {
  let mut map: Vec<Vec<Option<Letter>>> = s
    .lines()
    .map(|line| line.chars().map(Letter::from_char).collect())
    .collect();
  map.pop().unwrap();
  State { map }
}

pub fn p1(s: &str) -> usize {
  run(parse(s))
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  let s = include_str!("input/d23.txt");
  assert_eq!(p1(s), 13556);
  assert_eq!(p2(s), s.len());
}
