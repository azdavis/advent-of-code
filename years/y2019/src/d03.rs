use std::collections::HashSet;

pub fn p1(s: &str) -> i32 {
  let (fst, snd) = parse(s);
  evolve(&fst)
    .intersection(&evolve(&snd))
    .map(|p| p.x.abs() + p.y.abs())
    .min()
    .unwrap()
}

pub fn p2(_: &str) -> i32 {
  todo!()
}

fn evolve(xs: &[Action]) -> HashSet<Point> {
  let mut cur = Point { x: 0, y: 0 };
  let mut ret = HashSet::new();
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
      ret.insert(cur);
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
