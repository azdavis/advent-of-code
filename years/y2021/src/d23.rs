use helpers::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Letter {
  A,
  B,
  C,
  D,
}

impl Letter {
  fn from_char(c: char) -> Option<Letter> {
    let ret = match c {
      'A' => Letter::A,
      'B' => Letter::B,
      'C' => Letter::C,
      'D' => Letter::D,
      _ => return None,
    };
    Some(ret)
  }

  fn energy(&self) -> usize {
    match self {
      Letter::A => 1,
      Letter::B => 10,
      Letter::C => 100,
      Letter::D => 1000,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Loc {
  Hall(u8),
  Room(Letter, usize),
}

impl Loc {
  fn is_directly_outside_hallway(&self) -> bool {
    matches!(*self, Loc::Hall(2 | 4 | 6 | 8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pod {
  letter: Letter,
  idx: usize,
}

impl Pod {
  fn new(letter: Letter, idx: usize) -> Self {
    Self { letter, idx }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PodData {
  loc: Loc,
  locked: bool,
}

impl PodData {
  fn new(loc: Loc) -> Self {
    Self { loc, locked: false }
  }

  fn maybe_lock(&mut self) {
    if !self.locked && matches!(self.loc, Loc::Hall(_)) {
      self.locked = true;
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pods {
  a1: PodData,
  a2: PodData,
  b1: PodData,
  b2: PodData,
  c1: PodData,
  c2: PodData,
  d1: PodData,
  d2: PodData,
}

impl Pods {
  fn as_array(&self) -> [(Pod, PodData); 8] {
    [
      (Pod::new(Letter::A, 0), self.a1),
      (Pod::new(Letter::A, 1), self.a2),
      (Pod::new(Letter::B, 0), self.b1),
      (Pod::new(Letter::B, 1), self.b2),
      (Pod::new(Letter::C, 0), self.c1),
      (Pod::new(Letter::C, 1), self.c2),
      (Pod::new(Letter::D, 0), self.d1),
      (Pod::new(Letter::D, 1), self.d2),
    ]
  }

  fn as_mut_array(&mut self) -> [(Pod, &mut PodData); 8] {
    [
      (Pod::new(Letter::A, 0), &mut self.a1),
      (Pod::new(Letter::A, 1), &mut self.a2),
      (Pod::new(Letter::B, 0), &mut self.b1),
      (Pod::new(Letter::B, 1), &mut self.b2),
      (Pod::new(Letter::C, 0), &mut self.c1),
      (Pod::new(Letter::C, 1), &mut self.c2),
      (Pod::new(Letter::D, 0), &mut self.d1),
      (Pod::new(Letter::D, 1), &mut self.d2),
    ]
  }

  fn get(&self, pod: Pod) -> PodData {
    match (pod.letter, pod.idx) {
      (Letter::A, 0) => self.a1,
      (Letter::A, 1) => self.a2,
      (Letter::B, 0) => self.b1,
      (Letter::B, 1) => self.b2,
      (Letter::C, 0) => self.c1,
      (Letter::C, 1) => self.c2,
      (Letter::D, 0) => self.d1,
      (Letter::D, 1) => self.d2,
      _ => unreachable!(),
    }
  }

  fn set(&mut self, pod: Pod, data: PodData) {
    match (pod.letter, pod.idx) {
      (Letter::A, 0) => self.a1 = data,
      (Letter::A, 1) => self.a2 = data,
      (Letter::B, 0) => self.b1 = data,
      (Letter::B, 1) => self.b2 = data,
      (Letter::C, 0) => self.c1 = data,
      (Letter::C, 1) => self.c2 = data,
      (Letter::D, 0) => self.d1 = data,
      (Letter::D, 1) => self.d2 = data,
      _ => unreachable!(),
    }
  }

  fn maybe_lock_all(&mut self) {
    for (_, data) in self.as_mut_array() {
      data.maybe_lock();
    }
  }
}

impl fmt::Display for Pods {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "#############")?;
    let map: HashMap<_, _> = self
      .as_array()
      .into_iter()
      .map(|(pod, data)| (data.loc, pod.letter))
      .collect();
    let get_char = |loc: Loc| match map.get(&loc) {
      None => '.',
      Some(letter) => match letter {
        Letter::A => 'A',
        Letter::B => 'B',
        Letter::C => 'C',
        Letter::D => 'D',
      },
    };
    write!(f, "#")?;
    for i in 0..=HALL_WIDTH {
      write!(f, "{}", get_char(Loc::Hall(i)))?;
    }
    writeln!(f, "#")?;
    write!(f, "###")?;
    write!(f, "{}", get_char(Loc::Room(Letter::A, 0)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::B, 0)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::C, 0)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::D, 0)))?;
    writeln!(f, "###")?;
    write!(f, "  #")?;
    write!(f, "{}", get_char(Loc::Room(Letter::A, 1)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::B, 1)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::C, 1)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::D, 1)))?;
    writeln!(f, "#")?;
    write!(f, "  #########")?;
    Ok(())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MustMoveReason {
  OutsideHallway,
  MovingToFinalHallway,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
  pods: Pods,
  must_move: Option<(Pod, MustMoveReason)>,
}

const HALL_WIDTH: u8 = 10;

fn moves(loc: Loc) -> Vec<Loc> {
  match loc {
    Loc::Hall(n) => {
      let mut ret = Vec::with_capacity(1);
      if let Some(n_m1) = n.checked_sub(1) {
        ret.push(Loc::Hall(n_m1));
      }
      if n != HALL_WIDTH {
        ret.push(Loc::Hall(n + 1));
      }
      let letter = match n {
        2 => Letter::A,
        4 => Letter::B,
        6 => Letter::C,
        8 => Letter::D,
        _ => return ret,
      };
      ret.push(Loc::Room(letter, 0));
      ret
    }
    Loc::Room(letter, room_pos) => match room_pos {
      0 => {
        let hall_num = match letter {
          Letter::A => 2,
          Letter::B => 4,
          Letter::C => 6,
          Letter::D => 8,
        };
        vec![Loc::Hall(hall_num), Loc::Room(letter, 1)]
      }
      1 => vec![Loc::Room(letter, 0)],
      _ => unreachable!(),
    },
  }
}

fn mk_line(line: &str) -> impl Iterator<Item = [Letter; 2]> + '_ {
  line
    .chars()
    .filter_map(Letter::from_char)
    .enumerate()
    .map(|(idx, letter)| {
      let pos_letter = match idx {
        0 => Letter::A,
        1 => Letter::B,
        2 => Letter::C,
        3 => Letter::D,
        _ => panic!("bad idx"),
      };
      [letter, pos_letter]
    })
}

fn mk_pod(val: &(Letter, Letter, usize)) -> PodData {
  let &(_, letter, room_pos) = val;
  PodData::new(Loc::Room(letter, room_pos))
}

fn parse(s: &str) -> Pods {
  let mut lines = s.lines();
  assert_eq!(lines.next().unwrap().len(), 13);
  assert_eq!(lines.next().unwrap().len(), 13);
  let list: Vec<_> = std::iter::empty()
    .chain(mk_line(lines.next().unwrap()).map(|[a, b]| (a, b, 0)))
    .chain(mk_line(lines.next().unwrap()).map(|[a, b]| (a, b, 1)))
    .collect();
  let mut a = list.iter().filter(|&&(a, _, _)| matches!(a, Letter::A));
  let mut b = list.iter().filter(|&&(a, _, _)| matches!(a, Letter::B));
  let mut c = list.iter().filter(|&&(a, _, _)| matches!(a, Letter::C));
  let mut d = list.iter().filter(|&&(a, _, _)| matches!(a, Letter::D));
  Pods {
    a1: mk_pod(a.next().unwrap()),
    a2: mk_pod(a.next().unwrap()),
    b1: mk_pod(b.next().unwrap()),
    b2: mk_pod(b.next().unwrap()),
    c1: mk_pod(c.next().unwrap()),
    c2: mk_pod(c.next().unwrap()),
    d1: mk_pod(d.next().unwrap()),
    d2: mk_pod(d.next().unwrap()),
  }
}

fn maybe_add_new_state(
  visited: &mut HashMap<Pods, usize>,
  new_cur: &mut Vec<(State, usize)>,
  mut state: State,
  pod: Pod,
  loc: Loc,
  energy: usize,
) {
  state.pods.maybe_lock_all();
  state.pods.set(pod, PodData::new(loc));
  let new_energy = energy + pod.letter.energy();
  if let Some(&old_energy) = visited.get(&state.pods) {
    if new_energy >= old_energy {
      return;
    }
  }
  if is_in_final_loc(&state.pods, pod) {
    state.must_move = None;
  }
  visited.insert(state.pods, new_energy);
  new_cur.push((state, new_energy));
}

fn is_in_final_loc(pods: &Pods, pod: Pod) -> bool {
  match pods.get(pod).loc {
    Loc::Hall(_) => false,
    Loc::Room(room_letter, pos) => {
      if room_letter == pod.letter {
        match pos {
          0 => pods.as_array().into_iter().any(|(pod, data)| {
            pod.letter == room_letter && data.loc == Loc::Room(room_letter, 1)
          }),
          1 => true,
          _ => unreachable!(),
        }
      } else {
        false
      }
    }
  }
}

fn is_valid_move(
  pods: &Pods,
  letter: Letter,
  old_loc: Loc,
  new_loc: Loc,
) -> bool {
  if let (Loc::Hall(..), Loc::Room(room_letter, _)) = (old_loc, new_loc) {
    if letter != room_letter {
      return false;
    }
  }
  let ret = pods.as_array().iter().all(|&(_, data)| data.loc != new_loc);
  ret
}

pub fn p1(s: &str) -> usize {
  let pods = parse(s);
  let mut visited = HashMap::<Pods, usize>::default();
  visited.insert(pods, 0);
  let mut cur = vec![(
    State {
      pods,
      must_move: None,
    },
    0usize,
  )];
  loop {
    if cur.is_empty() {
      break;
    }
    let mut new_cur = Vec::<(State, usize)>::default();
    for (state, energy) in cur {
      match state.must_move {
        Some((pod, reason)) => {
          let data = state.pods.get(pod);
          for new_loc in moves(data.loc) {
            if !is_valid_move(&state.pods, pod.letter, data.loc, new_loc) {
              continue;
            }
            let mut new_state = state;
            if let MustMoveReason::OutsideHallway = reason {
              new_state.must_move = None;
            }
            maybe_add_new_state(
              &mut visited,
              &mut new_cur,
              new_state,
              pod,
              new_loc,
              energy,
            );
          }
        }
        None => {
          for (pod, data) in state.pods.as_array() {
            if is_in_final_loc(&state.pods, pod) {
              continue;
            }
            for new_loc in moves(data.loc) {
              if !is_valid_move(&state.pods, pod.letter, data.loc, new_loc) {
                continue;
              }
              let mut new_state = state;
              let reason = if data.locked {
                Some(MustMoveReason::MovingToFinalHallway)
              } else if new_loc.is_directly_outside_hallway() {
                Some(MustMoveReason::OutsideHallway)
              } else {
                None
              };
              new_state.must_move = reason.map(|r| (pod, r));
              maybe_add_new_state(
                &mut visited,
                &mut new_cur,
                new_state,
                pod,
                new_loc,
                energy,
              );
            }
          }
        }
      }
    }
    cur = new_cur;
  }
  visited
    .into_iter()
    .filter_map(|(pods, energy)| {
      let all_in_room =
        pods
          .as_array()
          .into_iter()
          .all(|(pod, data)| match data.loc {
            Loc::Hall(_) => false,
            Loc::Room(room_letter, _) => pod.letter == room_letter,
          });
      all_in_room.then(|| energy)
    })
    .min()
    .unwrap()
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
