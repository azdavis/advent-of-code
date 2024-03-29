use crate::intcode::{parse, Intcode};
use helpers::digits::{self, to_char, Digits};
use helpers::{hash_map, Compass, HashMap, HashSet};

type Vec2 = [i32; 2];

pub fn p1(s: &str) -> u32 {
  let scaffold = {
    let mut p = Intcode::parse(s);
    let mut output = Vec::new();
    assert!(p.run(&mut output).is_done());
    parse_screen(&output).scaffold
  };
  scaffold
    .iter()
    .filter_map(|&[x, y]| {
      neighbors([x, y])
        .iter()
        .all(|&n| scaffold.contains(&n))
        .then_some(u32::try_from(x * y).unwrap())
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
      .find_map(|(idx, &n)| (n == nl && output[idx.checked_sub(1)?] == nl).then_some(idx))
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
      for (m, c) in [
        (Move::TurnLeft, screen.facing.left()),
        (Move::TurnRight, screen.facing.right()),
      ] {
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
    let mut iter = decomposition.sequence.iter().map(|x| x.to_input());
    if let Some(x) = iter.next() {
      ret.push(x);
    }
    for x in iter {
      ret.push(b','.into());
      ret.push(x);
    }
    ret.push(b'\n'.into());
    for pat in [Pat::A, Pat::B, Pat::C] {
      write_moves(&decomposition.pats[&pat], &mut ret);
      ret.push(b'\n'.into());
    }
    ret.push(b'n'.into());
    ret.push(b'\n'.into());
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
  let mut iter = ms.iter().map(|x| x.to_input());
  if let Some(x) = iter.next() {
    buf.extend(x);
  }
  for x in iter {
    buf.push(b','.into());
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
  fn to_input(self) -> i64 {
    match self {
      Pat::A => b'A'.into(),
      Pat::B => b'B'.into(),
      Pat::C => b'C'.into(),
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
  let mut all_pats: Vec<_> = all_pats.into_iter().collect();
  all_pats.sort_by_cached_key(|&ms| {
    let count = (0..route.len())
      .filter(|&start| route.get(start..(start + ms.len())) == Some(ms))
      .count();
    std::cmp::Reverse(count * (ms.len() - 1))
  });
  let mut sequence = Vec::with_capacity(MAX_CHARS / 2);
  for &a in &all_pats {
    for &b in &all_pats {
      'outer: for &c in &all_pats {
        sequence.clear();
        let pats = hash_map([(a, Pat::A), (b, Pat::B), (c, Pat::C)]);
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
  let [x, y] = v;
  [[x, y - 1], [x - 1, y], [x + 1, y], [x, y + 1]]
}

fn neighbor(v: Vec2, compass: Compass) -> Vec2 {
  let [x, y] = v;
  // y increases to the south
  match compass {
    Compass::North => [x, y - 1],
    Compass::West => [x - 1, y],
    Compass::East => [x + 1, y],
    Compass::South => [x, y + 1],
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
  TurnLeft,
  TurnRight,
  Forward(u32),
}

impl Move {
  fn to_input(self) -> ToInput {
    match self {
      Move::TurnLeft => ToInput::Once(std::iter::once(b'L'.into())),
      Move::TurnRight => ToInput::Once(std::iter::once(b'R'.into())),
      Move::Forward(n) => ToInput::Digits(digits::get(n)),
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
  let mut scaffold = HashSet::default();
  let mut robot: Option<(Vec2, Compass)> = None;
  let mut x = 0i32;
  let mut y = 0i32;
  for &n in output.iter() {
    match u8::try_from(n).unwrap() {
      b'\n' => {
        x = 0;
        y += 1;
        continue;
      }
      b'#' => {
        scaffold.insert([x, y]);
      }
      b'.' => {}
      b'^' => robot = Some(([x, y], Compass::North)),
      b'v' => robot = Some(([x, y], Compass::South)),
      b'<' => robot = Some(([x, y], Compass::West)),
      b'>' => robot = Some(([x, y], Compass::East)),
      b => panic!("bad output: {b}"),
    }
    x += 1;
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
  assert_eq!(p2(s), 923_795);
}
