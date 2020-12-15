use std::collections::{HashMap, HashSet};

pub fn p1(s: &str) -> i32 {
  let (fst, snd) = parse(s);
  let fst_set: HashSet<_> = evolve(&fst).into_iter().map(|x| x.0).collect();
  let snd_set: HashSet<_> = evolve(&snd).into_iter().map(|x| x.0).collect();
  fst_set
    .intersection(&snd_set)
    .map(|p| p.x.abs() + p.y.abs())
    .min()
    .unwrap()
}

pub fn p2(s: &str) -> usize {
  let (fst, snd) = parse(s);
  let fst_map = evolve(&fst);
  let snd_map = evolve(&snd);
  let fst_set: HashSet<_> = fst_map.iter().map(|x| x.0).collect();
  let snd_set: HashSet<_> = snd_map.iter().map(|x| x.0).collect();
  fst_set
    .intersection(&snd_set)
    .map(|p| fst_map.get(&p).unwrap() + snd_map.get(&p).unwrap())
    .min()
    .unwrap()
}

fn evolve(xs: &[Action]) -> HashMap<Point, usize> {
  let mut cur = Point { x: 0, y: 0 };
  let mut ret = HashMap::new();
  let mut idx = 0;
  for ac in xs {
    let (dx, dy) = match ac.direction {
      Direction::Up => (0, 1),
      Direction::Down => (0, -1),
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
    };
    for _ in 0..ac.num {
      cur.x += dx;
      cur.y += dy;
      idx += 1;
      ret.entry(cur).or_insert(idx);
    }
  }
  ret
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32,
}

fn parse(s: &str) -> (Vec<Action>, Vec<Action>) {
  let mut lines = s.split('\n');
  let fst = parse_line(lines.next().unwrap());
  let snd = parse_line(lines.next().unwrap());
  (fst, snd)
}

fn parse_line(line: &str) -> Vec<Action> {
  line.split(',').map(Action::parse).collect()
}

struct Action {
  direction: Direction,
  num: u16,
}

impl Action {
  fn parse(s: &str) -> Self {
    let mut chars = s.chars();
    Action {
      direction: Direction::parse(chars.next().unwrap()),
      num: chars.as_str().parse().unwrap(),
    }
  }
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn parse(c: char) -> Self {
    match c {
      'U' => Self::Up,
      'D' => Self::Down,
      'L' => Self::Left,
      'R' => Self::Right,
      _ => panic!("bad char: {}", c),
    }
  }
}

#[test]
fn t() {
  let inp = include_str!("input/d03.txt");
  assert_eq!(p1(inp), 1626);
  assert_eq!(p2(inp), 27330);
}
