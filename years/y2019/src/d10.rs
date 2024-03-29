use helpers::{gcd, FloatOrd, HashMap, HashSet};
use std::cmp::Reverse;

type Vec2 = [i32; 2];

pub fn p1(s: &str) -> usize {
  let points = parse(s);
  let (_, count) = get_best(&points);
  count
}

pub fn p2(s: &str) -> u32 {
  let mut points = parse(s);
  let (best, _) = get_best(&points);
  assert!(points.remove(&best));
  let mut map = HashMap::<(i32, i32), Vec<Vec2>>::default();
  for &p in &points {
    map.entry(diff_gcd(best, p)).or_default().push(p);
  }
  for points in map.values_mut() {
    points.sort_unstable_by_key(|&p| Reverse(magnitude(best, p)));
  }
  let mut points: Vec<_> = map.into_iter().collect();
  points.sort_by_cached_key(|&((x, y), _)| {
    let mut ret = f64::from(x).atan2(f64::from(-y));
    if ret < 0.0 {
      ret += std::f64::consts::TAU;
    }
    FloatOrd(ret)
  });
  let mut idx = 1;
  loop {
    for (_, ps) in &mut points {
      if let Some([x, y]) = ps.pop() {
        if idx == 200 {
          return to_u32((x * 100) + y);
        }
        idx += 1;
      }
    }
    assert!(points.iter().any(|(_, ps)| !ps.is_empty()));
  }
}

fn get_best(points: &HashSet<Vec2>) -> (Vec2, usize) {
  points
    .iter()
    .map(|&a| {
      let count = points.iter().filter(|&&b| can_see(a, b, points)).count();
      (a, count)
    })
    .max_by_key(|&(_, count)| count)
    .unwrap()
}

fn parse(s: &str) -> HashSet<Vec2> {
  s.lines()
    .enumerate()
    .flat_map(|(y, line)| {
      line.chars().enumerate().filter_map(move |(x, c)| match c {
        '.' => None,
        '#' => Some([to_i32(x), to_i32(y)]),
        _ => panic!("bad tile: {c}"),
      })
    })
    .collect()
}

/// returns the square of the distance between `a` and `b`
fn magnitude(a: Vec2, b: Vec2) -> u32 {
  let [ax, ay] = a;
  let [bx, by] = b;
  to_u32((bx - ax).pow(2) + (by - ay).pow(2))
}

fn diff_gcd(a: Vec2, b: Vec2) -> (i32, i32) {
  let [ax, ay] = a;
  let [bx, by] = b;
  let dx = bx - ax;
  let dy = by - ay;
  let g = to_i32(gcd(to_usize(dx.abs()), to_usize(dy.abs())));
  (dx / g, dy / g)
}

/// returns whether `a` has a line of sight to `b` based on `points`
fn can_see(a: Vec2, b: Vec2, points: &HashSet<Vec2>) -> bool {
  if a == b {
    // special-case this
    return false;
  }
  let (dx, dy) = diff_gcd(a, b);
  let [mut x, mut y] = a;
  loop {
    x += dx;
    y += dy;
    if [x, y] == b {
      return true;
    }
    if points.contains(&[x, y]) {
      return false;
    }
  }
}

fn to_i32(n: usize) -> i32 {
  n.try_into().unwrap()
}

fn to_usize(n: i32) -> usize {
  n.try_into().unwrap()
}

fn to_u32(n: i32) -> u32 {
  n.try_into().unwrap()
}

#[test]
fn t() {
  let s = include_str!("input/d10.txt");
  assert_eq!(p1(s), 344);
  assert_eq!(p2(s), 2732);
}
