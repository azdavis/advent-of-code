use helpers::HashMap;

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

type Loc = [u8; 2];

#[derive(Debug, Clone)]
struct State {
  map: HashMap<Loc, Letter>,
}

impl State {
  fn desired_loc(&self, loc: Loc, mut bottom_row: u8) -> Loc {
    let &letter = self.map.get(&loc).unwrap();
    let want_col = letter_to_col(letter);
    loop {
      let want = [bottom_row, want_col];
      if want == loc {
        return want;
      }
      if let Some(&this_letter) = self.map.get(&want) {
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
    assert!(self.map.contains_key(&cur));
    let [mut cur_row, mut cur_col] = cur;
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
      if self.map.contains_key(&[cur_row, cur_col]) {
        return None;
      }
      steps += 1;
    }
    let mut ret = self.clone();
    let letter = ret.map.remove(&cur).unwrap();
    ret.map.insert(dst, letter);
    Some((ret, steps * letter.energy()))
  }

  fn all_locs(&self) -> impl Iterator<Item = Loc> + '_ {
    self.map.keys().copied()
  }
}

fn letter_to_col(letter: Letter) -> u8 {
  match letter {
    Letter::A => 3,
    Letter::B => 5,
    Letter::C => 7,
    Letter::D => 9,
  }
}

const HALLWAY_ROW: u8 = 1;
const HALLWAY_COLS: [u8; 7] = [1, 2, 4, 6, 8, 10, 11];

fn run(state: State, bottom_row: u8) -> usize {
  let mut ret = None::<usize>;
  let mut cur = vec![(state, 0usize)];
  while let Some((state, energy)) = cur.pop() {
    let mut done = true;
    for loc in state.all_locs() {
      let desired = state.desired_loc(loc, bottom_row);
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
  let map: HashMap<[u8; 2], Letter> = s
    .lines()
    .enumerate()
    .flat_map(|(row, line)| {
      line.chars().enumerate().filter_map(move |(col, c)| {
        Letter::from_char(c)
          .map(|l| ([row.try_into().unwrap(), col.try_into().unwrap()], l))
      })
    })
    .collect();
  State { map }
}

pub fn p1(s: &str) -> usize {
  run(parse(s), 3)
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
