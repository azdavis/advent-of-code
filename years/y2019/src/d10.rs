use helpers::float_ord::FloatOrd;
use helpers::gcd::gcd;
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
  let mut map = HashMap::<(i32, i32), Vec<Point>>::new();
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
      if let Some((x, y)) = ps.pop() {
        if idx == 200 {
          return to_u32((x * 100) + y);
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

type Point = (i32, i32);

fn get_best(points: &HashSet<Point>) -> (Point, usize) {
  points
    .iter()
    .map(|&a| {
      let count = points.iter().filter(|&&b| can_see(a, b, &points)).count();
      (a, count)
    })
    .max_by_key(|&(_, count)| count)
    .unwrap()
}

fn parse(s: &str) -> HashSet<Point> {
  s.lines()
    .enumerate()
    .flat_map(|(y, line)| {
      line.chars().enumerate().filter_map(move |(x, c)| match c {
        '.' => None,
        '#' => Some((to_i32(x), to_i32(y))),
        _ => panic!("bad tile: {}", c),
      })
    })
    .collect()
}

/// returns the square of the distance between `a` and `b`
fn magnitude(a: Point, b: Point) -> u32 {
  to_u32((b.0 - a.0).pow(2) + (b.1 - a.1).pow(2))
}

fn diff_gcd(a: Point, b: Point) -> (i32, i32) {
  let dx = b.0 - a.0;
  let dy = b.1 - a.1;
  let g = to_i32(gcd(to_usize(dx.abs()), to_usize(dy.abs())));
  (dx / g, dy / g)
}

/// returns whether `a` has a line of sight to `b` based on `points`
fn can_see(mut a: Point, b: Point, points: &HashSet<Point>) -> bool {
  if a == b {
    // special-case this
    return false;
  }
  let (dx, dy) = diff_gcd(a, b);
  loop {
    a.0 += dx;
    a.1 += dy;
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
  let inp = include_str!("input/d10.txt");
  assert_eq!(p1(inp), 344);
  assert_eq!(p2(inp), 2732);
}
