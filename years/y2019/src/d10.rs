use helpers::float_ord::FloatOrd;
use helpers::gcd::gcd;
use helpers::vec2::Vec2;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto as _;

pub fn p1(s: &str) -> usize {
  let points = parse(s);
  let (_, count) = get_best(&points);
  count
}

pub fn p2(s: &str) -> u32 {
  let mut points = parse(s);
  let (best, _) = get_best(&points);
  assert!(points.remove(&best));
  let mut map = HashMap::<(i32, i32), Vec<Vec2>>::new();
  for &p in points.iter() {
    map.entry(diff_gcd(best, p)).or_default().push(p);
  }
  for points in map.values_mut() {
    points.sort_unstable_by_key(|&p| Reverse(magnitude(best, p)));
  }
  let mut points: Vec<_> = map.into_iter().collect();
  points.sort_by_cached_key(|&((x, y), _)| {
    let mut ret = (x as f64).atan2(-y as f64);
    if ret < 0.0 {
      ret += std::f64::consts::TAU;
    }
    FloatOrd(ret)
  });
  let mut idx = 1;
  loop {
    for (_, ps) in points.iter_mut() {
      if let Some(p) = ps.pop() {
        if idx == 200 {
          return to_u32((p.x * 100) + p.y);
        } else {
          idx += 1;
        }
      }
    }
    if points.iter().all(|(_, ps)| ps.is_empty()) {
      panic!("no solution")
    }
  }
}

fn get_best(points: &HashSet<Vec2>) -> (Vec2, usize) {
  points
    .iter()
    .map(|&a| {
      let count = points.iter().filter(|&&b| can_see(a, b, &points)).count();
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
        '#' => Some(Vec2::new(to_i32(x), to_i32(y))),
        _ => panic!("bad tile: {}", c),
      })
    })
    .collect()
}

/// returns the square of the distance between `a` and `b`
fn magnitude(a: Vec2, b: Vec2) -> u32 {
  to_u32((b.x - a.x).pow(2) + (b.y - a.y).pow(2))
}

fn diff_gcd(a: Vec2, b: Vec2) -> (i32, i32) {
  let dx = b.x - a.x;
  let dy = b.y - a.y;
  let g = to_i32(gcd(to_usize(dx.abs()), to_usize(dy.abs())));
  (dx / g, dy / g)
}

/// returns whether `a` has a line of sight to `b` based on `points`
fn can_see(mut a: Vec2, b: Vec2, points: &HashSet<Vec2>) -> bool {
  if a == b {
    // special-case this
    return false;
  }
  let (dx, dy) = diff_gcd(a, b);
  loop {
    a.x += dx;
    a.y += dy;
    if a == b {
      return true;
    }
    if points.contains(&a) {
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
