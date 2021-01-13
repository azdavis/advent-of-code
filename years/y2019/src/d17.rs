use crate::intcode::{parse, Intcode};
use helpers::compass::Compass;
use helpers::digits::{digits, to_char, Digits};
use helpers::maplit::hashmap;
use helpers::vec2::Vec2;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom as _;

pub fn p1(s: &str) -> u32 {
  let scaffold = {
    let mut p = Intcode::parse(s);
    let mut output = Vec::new();
    assert!(p.run(&mut output).is_done());
    parse_screen(&output).scaffold
  };
  scaffold
    .iter()
    .filter_map(|&sc| {
      let all_scaffold = neighbors(sc).iter().all(|&n| scaffold.contains(&n));
      if all_scaffold {
        Some((sc.x * sc.y) as u32)
      } else {
        None
      }
    })
    .sum()
}

pub fn p2(s: &str) -> i64 {
  let mut p = {
    let mut ns: Vec<_> = parse(s).collect();
    assert_eq!(ns[0], 1);
    ns[0] = 2;
    Intcode::new(ns)
  };
  let mut output = Vec::new();
  let mut screen = {
    assert!(p.run(&mut output).needs_input());
    let nl = i64::from(b'\n');
    let idx = output
      .iter()
      .enumerate()
      .find_map(|(idx, &n)| {
        if n == nl && output[idx.checked_sub(1)?] == nl {
          Some(idx)
        } else {
          None
        }
      })
      .unwrap();
    output.truncate(idx);
    parse_screen(&output)
  };
  let route = {
    let mut ret = Vec::new();
    let mut cur_move = 0;
    'outer: loop {
      // always try to go forward first
      let forward = neighbor(screen.loc, screen.facing);
      if screen.scaffold.contains(&forward) {
        screen.loc = forward;
        cur_move += 1;
        continue 'outer;
      }
      // then try turning
      for &(m, c) in [
        (Move::TurnLeft, screen.facing.left()),
        (Move::TurnRight, screen.facing.right()),
      ]
      .iter()
      {
        let loc = neighbor(screen.loc, c);
        if screen.scaffold.contains(&loc) {
          screen.loc = loc;
          screen.facing = c;
          // for the first turn
          if cur_move != 0 {
            ret.push(Move::Forward(cur_move));
          }
          ret.push(m);
          cur_move = 1;
          continue 'outer;
        }
      }
      ret.push(Move::Forward(cur_move));
      break;
    }
    ret
  };
  let decomposition = decompose(&route).unwrap();
  let inp = {
    let mut ret = Vec::new();
    let mut iter = decomposition.sequence.iter().map(Pat::to_input);
    if let Some(x) = iter.next() {
      ret.push(x);
    }
    for x in iter {
      ret.push(b',' as i64);
      ret.push(x);
    }
    ret.push(b'\n' as i64);
    for pat in [Pat::A, Pat::B, Pat::C].iter() {
      write_moves(&decomposition.pats[pat], &mut ret);
      ret.push(b'\n' as i64);
    }
    ret.push(b'n' as i64);
    ret.push(b'\n' as i64);
    ret
  };
  for x in inp {
    p.input(x);
  }
  output.clear();
  assert!(p.run(&mut output).is_done());
  output.pop().unwrap()
}

fn write_moves(ms: &[Move], buf: &mut Vec<i64>) {
  let mut iter = ms.iter().map(Move::to_input);
  if let Some(x) = iter.next() {
    buf.extend(x);
  }
  for x in iter {
    buf.push(b',' as i64);
    buf.extend(x);
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pat {
  A,
  B,
  C,
}

impl Pat {
  fn to_input(&self) -> i64 {
    match *self {
      Pat::A => b'A' as i64,
      Pat::B => b'B' as i64,
      Pat::C => b'C' as i64,
    }
  }
}

#[derive(Debug)]
struct Res {
  pats: HashMap<Pat, Vec<Move>>,
  sequence: Vec<Pat>,
}

const MAX_CHARS: usize = 20;

fn decompose(route: &[Move]) -> Option<Res> {
  let all_pats: HashSet<&[Move]> = (0..route.len())
    .flat_map(|start| (2_usize..=(MAX_CHARS / 2)).map(move |len| (start, len)))
    .filter_map(|(start, len)| route.get(start..(start + len)))
    .filter(|ms| {
      let mut buf = Vec::with_capacity(MAX_CHARS);
      write_moves(ms, &mut buf);
      buf.len() <= MAX_CHARS
    })
    .collect();
  let mut sequence = Vec::with_capacity(MAX_CHARS / 2);
  for &a in all_pats.iter() {
    for &b in all_pats.iter() {
      'outer: for &c in all_pats.iter() {
        sequence.clear();
        let pats = hashmap![a => Pat::A, b => Pat::B, c => Pat::C];
        if pats.len() != 3 {
          // duplicate
          continue 'outer;
        }
        let mut idx = 0;
        loop {
          if idx == route.len() {
            return Some(Res {
              pats: pats
                .into_iter()
                .map(|(ms, pat)| (pat, ms.to_owned()))
                .collect(),
              sequence,
            });
          }
          let next = pats
            .iter()
            .find(|&(&ms, _)| route.get(idx..(idx + ms.len())) == Some(ms));
          match next {
            None => continue 'outer,
            Some((&ms, &pat)) => {
              idx += ms.len();
              sequence.push(pat);
            }
          }
          if sequence.len() > MAX_CHARS / 2 {
            continue 'outer;
          }
        }
      }
    }
  }
  None
}

fn neighbors(v: Vec2) -> [Vec2; 4] {
  let x = v.x;
  let y = v.y;
  [
    Vec2::new(x, y - 1),
    Vec2::new(x - 1, y),
    Vec2::new(x + 1, y),
    Vec2::new(x, y + 1),
  ]
}

fn neighbor(v: Vec2, compass: Compass) -> Vec2 {
  let x = v.x;
  let y = v.y;
  // y increases to the south
  match compass {
    Compass::North => Vec2::new(x, y - 1),
    Compass::West => Vec2::new(x - 1, y),
    Compass::East => Vec2::new(x + 1, y),
    Compass::South => Vec2::new(x, y + 1),
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
  TurnLeft,
  TurnRight,
  Forward(u32),
}

impl Move {
  fn to_input(&self) -> ToInput {
    match *self {
      Move::TurnLeft => ToInput::Once(std::iter::once(b'L' as i64)),
      Move::TurnRight => ToInput::Once(std::iter::once(b'R' as i64)),
      Move::Forward(n) => ToInput::Digits(digits(n)),
    }
  }
}

#[derive(Debug)]
enum ToInput {
  Once(std::iter::Once<i64>),
  Digits(Digits),
}

impl Iterator for ToInput {
  type Item = i64;
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      ToInput::Once(once) => once.next(),
      ToInput::Digits(ds) => ds.next().map(|d| to_char(d) as i64),
    }
  }
}

#[derive(Debug)]
struct Screen {
  scaffold: HashSet<Vec2>,
  loc: Vec2,
  facing: Compass,
}

fn parse_screen(output: &[i64]) -> Screen {
  let mut scaffold = HashSet::new();
  let mut robot: Option<(Vec2, Compass)> = None;
  let mut cur = Vec2::default();
  for &n in output.iter() {
    match u8::try_from(n).unwrap() {
      b'\n' => {
        cur.x = 0;
        cur.y += 1;
        continue;
      }
      b'#' => {
        scaffold.insert(cur);
      }
      b'.' => {}
      b'^' => robot = Some((cur, Compass::North)),
      b'v' => robot = Some((cur, Compass::South)),
      b'<' => robot = Some((cur, Compass::West)),
      b'>' => robot = Some((cur, Compass::East)),
      b => panic!("bad output: {}", b),
    }
    cur.x += 1;
  }
  let (loc, facing) = robot.unwrap();
  Screen {
    scaffold,
    loc,
    facing,
  }
}

#[test]
fn t() {
  let s = include_str!("input/d17.txt");
  assert_eq!(p1(s), 5940);
  assert_eq!(p2(s), 923795);
}
