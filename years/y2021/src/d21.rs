use helpers::{static_regex, Counter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
  pos: usize,
  score: usize,
}

impl Player {
  fn new(pos: usize) -> Self {
    Self { pos, score: 0 }
  }

  fn update(&mut self, roll: usize) {
    self.pos = ((self.pos + roll - 1) % 10) + 1;
    self.score += self.pos;
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
  p1: Player,
  p2: Player,
}

static_regex!(RE = r"^Player \d+ starting position: (\d+)$");

fn pos(s: &str) -> usize {
  RE.captures(s)
    .unwrap()
    .get(1)
    .unwrap()
    .as_str()
    .parse()
    .unwrap()
}

fn parse(s: &str) -> State {
  let mut lines = s.lines();
  let p1_pos = pos(lines.next().unwrap());
  let p2_pos = pos(lines.next().unwrap());
  assert!(lines.next().is_none());
  State {
    p1: Player::new(p1_pos),
    p2: Player::new(p2_pos),
  }
}

struct P1Die {
  idx: usize,
  rolls: usize,
}

impl P1Die {
  fn next(&mut self) -> usize {
    let mut ret = 0usize;
    for _ in 0..3 {
      ret += self.idx;
      self.idx = (self.idx % 100) + 1;
      self.rolls += 1;
    }
    ret
  }
}

impl Default for P1Die {
  fn default() -> Self {
    Self { idx: 1, rolls: 0 }
  }
}

const P1_CUTOFF: usize = 1000;

pub fn p1(s: &str) -> usize {
  let mut state = parse(s);
  let mut die = P1Die::default();
  loop {
    state.p1.update(die.next());
    if state.p1.score >= P1_CUTOFF {
      return state.p2.score * die.rolls;
    }
    state.p2.update(die.next());
    if state.p2.score >= P1_CUTOFF {
      return state.p1.score * die.rolls;
    }
  }
}

fn get_counts(dice: usize, sum: usize, counts: &mut Counter<usize>) {
  if dice == 0 {
    counts.inc(sum);
    return;
  }
  for r in 1..=3 {
    get_counts(dice - 1, sum + r, counts);
  }
}

const P2_CUTOFF: usize = 21;

pub fn p2(s: &str) -> usize {
  let state = parse(s);
  let rolls = {
    let mut ret = Counter::<usize>::default();
    get_counts(3, 0, &mut ret);
    ret
  };
  let mut states = Counter::<State>::default();
  states.inc(state);
  let mut p1_win = 0usize;
  let mut p2_win = 0usize;
  while !states.is_empty() {
    let mut new_states = Counter::<State>::default();
    for (state, state_count) in states {
      for (&roll, p1_roll_count) in rolls.iter() {
        let mut state = state;
        state.p1.update(roll);
        if state.p1.score >= P2_CUTOFF {
          p1_win += state_count * p1_roll_count;
          continue;
        }
        for (&roll, p2_roll_count) in rolls.iter() {
          let mut state = state;
          state.p2.update(roll);
          if state.p2.score >= P2_CUTOFF {
            p2_win += state_count * p1_roll_count * p2_roll_count;
            continue;
          }
          new_states.add(state, state_count * p1_roll_count * p2_roll_count);
        }
      }
    }
    states = new_states;
  }
  p1_win.max(p2_win)
}

#[test]
fn t() {
  let s = include_str!("input/d21.txt");
  assert_eq!(p1(s), 1067724);
  assert_eq!(p2(s), 630947104784464);
}

#[test]
fn ex1() {
  let s = include_str!("input/d21_ex1.txt");
  assert_eq!(p1(s), 739785);
  assert_eq!(p2(s), 444356092776315);
}
