use helpers::{Compass, HashMap, HashSet};

type Vec2 = [i32; 2];

pub fn p1(s: &str) -> i32 {
  let [fst, snd] = parse(s);
  let fst_set: HashSet<_> = evolve(&fst).into_iter().map(|x| x.0).collect();
  let snd_set: HashSet<_> = evolve(&snd).into_iter().map(|x| x.0).collect();
  fst_set
    .intersection(&snd_set)
    .map(|&[x, y]| x.abs() + y.abs())
    .min()
    .unwrap()
}

pub fn p2(s: &str) -> usize {
  let [fst, snd] = parse(s);
  let fst_map = evolve(&fst);
  let snd_map = evolve(&snd);
  let fst_set: HashSet<_> = fst_map.iter().map(|x| x.0).collect();
  let snd_set: HashSet<_> = snd_map.iter().map(|x| x.0).collect();
  fst_set
    .intersection(&snd_set)
    .map(|&p| fst_map.get(p).unwrap() + snd_map.get(p).unwrap())
    .min()
    .unwrap()
}

fn evolve(xs: &[Action]) -> HashMap<Vec2, usize> {
  let mut x = 0i32;
  let mut y = 0i32;
  let mut ret = HashMap::default();
  let mut idx = 0;
  for ac in xs {
    let [dx, dy] = ac.direction.dx_dy();
    for _ in 0..ac.num {
      x += dx;
      y += dy;
      idx += 1;
      ret.entry([x, y]).or_insert(idx);
    }
  }
  ret
}

fn parse(s: &str) -> [Vec<Action>; 2] {
  let mut lines = s.lines();
  let fst = parse_line(lines.next().unwrap());
  let snd = parse_line(lines.next().unwrap());
  [fst, snd]
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
    _ => panic!("bad char: {c}"),
  }
}

#[test]
fn t() {
  let s = include_str!("input/d03.txt");
  assert_eq!(p1(s), 1626);
  assert_eq!(p2(s), 27330);
}
