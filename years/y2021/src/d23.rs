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
enum RoomLoc {
  N0,
  N1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HallLoc {
  N0,
  N1,
  N2,
  N3,
  N4,
  N5,
  N6,
  N7,
  N8,
  N9,
  N10,
}

impl HallLoc {
  fn inc(self) -> Option<Self> {
    let ret = match self {
      HallLoc::N0 => HallLoc::N1,
      HallLoc::N1 => HallLoc::N2,
      HallLoc::N2 => HallLoc::N3,
      HallLoc::N3 => HallLoc::N4,
      HallLoc::N4 => HallLoc::N5,
      HallLoc::N5 => HallLoc::N6,
      HallLoc::N6 => HallLoc::N7,
      HallLoc::N7 => HallLoc::N8,
      HallLoc::N8 => HallLoc::N9,
      HallLoc::N9 => HallLoc::N10,
      HallLoc::N10 => return None,
    };
    Some(ret)
  }

  fn dec(self) -> Option<Self> {
    let ret = match self {
      HallLoc::N0 => return None,
      HallLoc::N1 => HallLoc::N0,
      HallLoc::N2 => HallLoc::N1,
      HallLoc::N3 => HallLoc::N2,
      HallLoc::N4 => HallLoc::N3,
      HallLoc::N5 => HallLoc::N4,
      HallLoc::N6 => HallLoc::N5,
      HallLoc::N7 => HallLoc::N6,
      HallLoc::N8 => HallLoc::N7,
      HallLoc::N9 => HallLoc::N8,
      HallLoc::N10 => HallLoc::N9,
    };
    Some(ret)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Loc {
  Hall(HallLoc),
  Room(Letter, RoomLoc),
}

impl Loc {
  fn is_directly_outside_hallway(&self) -> bool {
    matches!(
      *self,
      Loc::Hall(HallLoc::N2 | HallLoc::N4 | HallLoc::N6 | HallLoc::N8)
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Idx {
  N0,
  N1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pod {
  letter: Letter,
  idx: Idx,
}

impl Pod {
  fn new(letter: Letter, idx: Idx) -> Self {
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
      (Pod::new(Letter::A, Idx::N0), self.a1),
      (Pod::new(Letter::A, Idx::N1), self.a2),
      (Pod::new(Letter::B, Idx::N0), self.b1),
      (Pod::new(Letter::B, Idx::N1), self.b2),
      (Pod::new(Letter::C, Idx::N0), self.c1),
      (Pod::new(Letter::C, Idx::N1), self.c2),
      (Pod::new(Letter::D, Idx::N0), self.d1),
      (Pod::new(Letter::D, Idx::N1), self.d2),
    ]
  }

  fn as_mut_array(&mut self) -> [(Pod, &mut PodData); 8] {
    [
      (Pod::new(Letter::A, Idx::N0), &mut self.a1),
      (Pod::new(Letter::A, Idx::N1), &mut self.a2),
      (Pod::new(Letter::B, Idx::N0), &mut self.b1),
      (Pod::new(Letter::B, Idx::N1), &mut self.b2),
      (Pod::new(Letter::C, Idx::N0), &mut self.c1),
      (Pod::new(Letter::C, Idx::N1), &mut self.c2),
      (Pod::new(Letter::D, Idx::N0), &mut self.d1),
      (Pod::new(Letter::D, Idx::N1), &mut self.d2),
    ]
  }

  fn get(&self, pod: Pod) -> PodData {
    match (pod.letter, pod.idx) {
      (Letter::A, Idx::N0) => self.a1,
      (Letter::A, Idx::N1) => self.a2,
      (Letter::B, Idx::N0) => self.b1,
      (Letter::B, Idx::N1) => self.b2,
      (Letter::C, Idx::N0) => self.c1,
      (Letter::C, Idx::N1) => self.c2,
      (Letter::D, Idx::N0) => self.d1,
      (Letter::D, Idx::N1) => self.d2,
    }
  }

  fn set(&mut self, pod: Pod, data: PodData) {
    match (pod.letter, pod.idx) {
      (Letter::A, Idx::N0) => self.a1 = data,
      (Letter::A, Idx::N1) => self.a2 = data,
      (Letter::B, Idx::N0) => self.b1 = data,
      (Letter::B, Idx::N1) => self.b2 = data,
      (Letter::C, Idx::N0) => self.c1 = data,
      (Letter::C, Idx::N1) => self.c2 = data,
      (Letter::D, Idx::N0) => self.d1 = data,
      (Letter::D, Idx::N1) => self.d2 = data,
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
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N0)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N1)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N2)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N3)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N4)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N5)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N6)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N7)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N8)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N9)))?;
    write!(f, "{}", get_char(Loc::Hall(HallLoc::N10)))?;
    writeln!(f, "#")?;
    write!(f, "###")?;
    write!(f, "{}", get_char(Loc::Room(Letter::A, RoomLoc::N0)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::B, RoomLoc::N0)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::C, RoomLoc::N0)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::D, RoomLoc::N0)))?;
    writeln!(f, "###")?;
    write!(f, "  #")?;
    write!(f, "{}", get_char(Loc::Room(Letter::A, RoomLoc::N1)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::B, RoomLoc::N1)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::C, RoomLoc::N1)))?;
    write!(f, "#")?;
    write!(f, "{}", get_char(Loc::Room(Letter::D, RoomLoc::N1)))?;
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

fn moves(loc: Loc) -> Vec<Loc> {
  match loc {
    Loc::Hall(n) => {
      let mut ret = Vec::with_capacity(1);
      if let Some(n_m1) = n.dec() {
        ret.push(Loc::Hall(n_m1));
      }
      if let Some(n_p1) = n.inc() {
        ret.push(Loc::Hall(n_p1));
      }
      let letter = match n {
        HallLoc::N2 => Letter::A,
        HallLoc::N4 => Letter::B,
        HallLoc::N6 => Letter::C,
        HallLoc::N8 => Letter::D,
        _ => return ret,
      };
      ret.push(Loc::Room(letter, RoomLoc::N0));
      ret
    }
    Loc::Room(letter, room_pos) => match room_pos {
      RoomLoc::N0 => {
        let hall_num = match letter {
          Letter::A => HallLoc::N2,
          Letter::B => HallLoc::N4,
          Letter::C => HallLoc::N6,
          Letter::D => HallLoc::N8,
        };
        vec![Loc::Hall(hall_num), Loc::Room(letter, RoomLoc::N1)]
      }
      RoomLoc::N1 => vec![Loc::Room(letter, RoomLoc::N0)],
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

fn mk_pod(val: &(Letter, Letter, RoomLoc)) -> PodData {
  let &(_, letter, room_pos) = val;
  PodData::new(Loc::Room(letter, room_pos))
}

fn parse(s: &str) -> Pods {
  let mut lines = s.lines();
  assert_eq!(lines.next().unwrap().len(), 13);
  assert_eq!(lines.next().unwrap().len(), 13);
  let list: Vec<_> = std::iter::empty()
    .chain(mk_line(lines.next().unwrap()).map(|[a, b]| (a, b, RoomLoc::N0)))
    .chain(mk_line(lines.next().unwrap()).map(|[a, b]| (a, b, RoomLoc::N1)))
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
          RoomLoc::N0 => pods.as_array().into_iter().any(|(pod, data)| {
            pod.letter == room_letter
              && data.loc == Loc::Room(room_letter, RoomLoc::N1)
          }),
          RoomLoc::N1 => true,
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
