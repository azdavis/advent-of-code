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

type Loc = [usize; 2];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
  map: Vec<Vec<Option<Letter>>>,
}

impl State {
  fn desired_loc(&self, loc: Loc) -> Loc {
    let letter = self.map[loc[0]][loc[1]].unwrap();
    let want_col: usize = match letter {
      Letter::A => 3,
      Letter::B => 5,
      Letter::C => 7,
      Letter::D => 9,
    };
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

const HALLWAY_ROW: usize = 0;
const HALLWAY_COLS: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];

struct CacheStack {
  cur: Vec<(State, usize)>,
  cache: HashMap<State, usize>,
}

impl CacheStack {
  fn new(init: State) -> Self {
    let cur = vec![(init, 0usize)];
    let cache: HashMap<_, _> = cur.iter().cloned().collect();
    Self { cur, cache }
  }

  fn add(&mut self, energy: usize, state: &State, cur: Loc, dst: Loc) -> bool {
    let (new_state, add_energy) = match state.try_move(cur, dst) {
      Some(x) => x,
      None => return false,
    };
    let new_energy = energy + add_energy;
    if let Some(&old_energy) = self.cache.get(&new_state) {
      if old_energy <= new_energy {
        return false;
      }
    }
    self.cache.insert(new_state.clone(), new_energy);
    self.cur.push((new_state, new_energy));
    true
  }
}

fn run(state: State) -> usize {
  let mut ret = None::<usize>;
  let mut runner = CacheStack::new(state);
  while let Some((state, energy)) = runner.cur.pop() {
    let mut done = true;
    for loc in state.all_locs() {
      let desired = state.desired_loc(loc);
      if loc == desired {
        continue;
      }
      done = false;
      if runner.add(energy, &state, loc, desired) {
        continue;
      }
      if loc[0] == HALLWAY_ROW {
        continue;
      }
      for col in HALLWAY_COLS {
        runner.add(energy, &state, loc, [HALLWAY_ROW, col]);
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

fn parse_line(line: &str) -> Vec<Option<Letter>> {
  line.chars().map(Letter::from_char).collect()
}

fn parse(s: &str) -> State {
  let mut map: Vec<_> = s.lines().skip(1).map(parse_line).collect();
  map.pop().unwrap();
  State { map }
}

pub fn p1(s: &str) -> usize {
  run(parse(s))
}

pub fn p2(s: &str) -> usize {
  let mut state = parse(s);
  let mut extra = include_str!("d23_p2_extra.txt").lines().map(parse_line);
  state.map.insert(2, extra.next().unwrap());
  state.map.insert(3, extra.next().unwrap());
  assert!(extra.next().is_none());
  run(state)
}

#[test]
fn t() {
  let s = include_str!("input/d23.txt");
  assert_eq!(p1(s), 13556);
  assert_eq!(p2(s), 54200);
}
