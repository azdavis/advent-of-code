use helpers::gcd::gcd;
use std::collections::HashSet;
use std::convert::TryInto as _;

pub fn p1(s: &str) -> usize {
  let asteroids: HashSet<Point> = s
    .lines()
    .enumerate()
    .flat_map(|(y, line)| {
      line.chars().enumerate().filter_map(move |(x, c)| match c {
        '.' => None,
        '#' => Some((to_i32(x), to_i32(y))),
        _ => panic!("bad tile: {}", c),
      })
    })
    .collect();
  asteroids
    .iter()
    .map(|&a| {
      asteroids
        .iter()
        .filter(|&&b| can_see(a, b, &asteroids))
        .count()
    })
    .max()
    .unwrap()
}

pub fn p2(s: &str) -> u32 {
  todo!()
}

type Point = (i32, i32);

fn diff_gcd(a: Point, b: Point) -> (i32, i32) {
  let (ax, ay) = a;
  let (bx, by) = b;
  let dx = bx - ax;
  let dy = by - ay;
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

#[test]
fn t() {
  let inp = include_str!("input/d10.txt");
  assert_eq!(p1(inp), 344);
  // assert_eq!(p2(inp), ___);
}
