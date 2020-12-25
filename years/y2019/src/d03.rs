use helpers::compass::Compass;
use helpers::vec2::Vec2;
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

fn evolve(xs: &[Action]) -> HashMap<Vec2, usize> {
  let mut cur = Vec2::default();
  let mut ret = HashMap::new();
  let mut idx = 0;
  for ac in xs {
    let (dx, dy) = ac.direction.dx_dy();
    for _ in 0..ac.num {
      cur.x += dx;
      cur.y += dy;
      idx += 1;
      ret.entry(cur).or_insert(idx);
    }
  }
  ret
}

fn parse(s: &str) -> (Vec<Action>, Vec<Action>) {
  let mut lines = s.lines();
  let fst = parse_line(lines.next().unwrap());
  let snd = parse_line(lines.next().unwrap());
  (fst, snd)
}

fn parse_line(line: &str) -> Vec<Action> {
  line.split(',').map(Action::parse).collect()
}

struct Action {
  direction: Compass,
  num: u16,
}

impl Action {
  fn parse(s: &str) -> Self {
    let mut chars = s.chars();
    Action {
      direction: parse_compass(chars.next().unwrap()),
      num: chars.as_str().parse().unwrap(),
    }
  }
}

fn parse_compass(c: char) -> Compass {
  match c {
    'U' => Compass::North,
    'D' => Compass::South,
    'L' => Compass::West,
    'R' => Compass::East,
    _ => panic!("bad char: {}", c),
  }
}

#[test]
fn t() {
  let inp = include_str!("input/d03.txt");
  assert_eq!(p1(inp), 1626);
  assert_eq!(p2(inp), 27330);
}
