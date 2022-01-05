use helpers::HashMap;

type Vec2 = [isize; 2];
type Cube = [Vec2; 3];

fn parse_vec2(s: &str, axis: &str) -> Vec2 {
  let (got_axis, ns) = s.split_once('=').unwrap();
  assert_eq!(axis, got_axis);
  let (start, end) = ns.split_once("..").unwrap();
  [start.parse().unwrap(), end.parse().unwrap()]
}

fn parse(s: &str) -> impl Iterator<Item = (bool, Cube)> + '_ {
  s.lines().map(|line| {
    let (action, coords) = line.split_once(' ').unwrap();
    let on = match action {
      "on" => true,
      "off" => false,
      _ => panic!("unknown action: {}", action),
    };
    let mut coords = coords.split(',');
    let x = parse_vec2(coords.next().unwrap(), "x");
    let y = parse_vec2(coords.next().unwrap(), "y");
    let z = parse_vec2(coords.next().unwrap(), "z");
    assert!(coords.next().is_none());
    (on, [x, y, z])
  })
}

/// from https://redd.it/rlxhmg
fn run<I>(iter: I) -> isize
where
  I: Iterator<Item = (bool, Cube)>,
{
  let mut cubes = HashMap::<Cube, isize>::default();
  for (on, cube) in iter {
    let [[a_sx, a_ex], [a_sy, a_ey], [a_sz, a_ez]] = cube;
    let mut update = HashMap::<Cube, isize>::default();
    for (&[[b_sx, b_ex], [b_sy, b_ey], [b_sz, b_ez]], &count) in cubes.iter() {
      let sx = a_sx.max(b_sx);
      let ex = a_ex.min(b_ex);
      let sy = a_sy.max(b_sy);
      let ey = a_ey.min(b_ey);
      let sz = a_sz.max(b_sz);
      let ez = a_ez.min(b_ez);
      if sx <= ex && sy <= ey && sz <= ez {
        *update.entry([[sx, ex], [sy, ey], [sz, ez]]).or_default() -= count;
      }
    }
    if on {
      *update.entry(cube).or_default() += 1;
    }
    for (cube, count) in update {
      *cubes.entry(cube).or_default() += count;
    }
  }
  cubes
    .into_iter()
    .map(|([[sx, ex], [sy, ey], [sz, ez]], count)| {
      (ex - sx + 1) * (ey - sy + 1) * (ez - sz + 1) * count
    })
    .sum()
}

const BOUND: isize = 50;

pub fn p1(s: &str) -> isize {
  run(parse(s).filter(|&(_, cube)| {
    cube
      .into_iter()
      .flatten()
      .all(|it| (-BOUND..=BOUND).contains(&it))
  }))
}

pub fn p2(s: &str) -> isize {
  run(parse(s))
}

#[test]
fn t() {
  let s = include_str!("input/d22.txt");
  assert_eq!(p1(s), 547648);
  assert_eq!(p2(s), 1206644425246111);
}
