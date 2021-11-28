use helpers::Infinitable;
use std::cmp::Ordering;

type Vec2 = [usize; 2];

fn parse(s: &str) -> (Vec<Vec2>, usize, usize) {
  let points: Vec<Vec2> = s
    .lines()
    .map(|line| {
      let (x, y) = line.split_once(", ").unwrap();
      [x.parse().unwrap(), y.parse().unwrap()]
    })
    .collect();
  let width = points.iter().map(|&[x, _]| x).max().unwrap();
  let height = points.iter().map(|&[_, y]| y).max().unwrap();
  (points, width, height)
}

fn city_dist([xa, ya]: Vec2, [xb, yb]: Vec2) -> usize {
  (xa.max(xb) - xa.min(xb)) + (ya.max(yb) - ya.min(yb))
}

/// returns the index of the unique point with minimum distance to `p` or None
/// if no unique point exists.
fn min_point(points: &[Vec2], p: Vec2) -> Option<usize> {
  let mut iter = points.iter().enumerate();
  let (id, &p2) = iter.next().unwrap();
  let mut min_dist = city_dist(p, p2);
  let mut min_id = id;
  let mut count = 1usize;
  for (id, &p2) in iter {
    let dist = city_dist(p, p2);
    match dist.cmp(&min_dist) {
      Ordering::Less => {
        min_dist = dist;
        min_id = id;
        count = 1;
      }
      Ordering::Equal => count += 1,
      Ordering::Greater => {}
    }
  }
  (count == 1).then(|| min_id)
}

pub fn p1(s: &str) -> usize {
  let (points, width, height) = parse(s);
  let mut sizes = vec![Infinitable::Finite(0usize); points.len()];
  for y in 0..height {
    for x in 0..width {
      if let Some(idx) = min_point(&points, [x, y]) {
        if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
          sizes[idx] = Infinitable::PosInf;
        } else {
          sizes[idx] += 1;
        }
      }
    }
  }
  sizes.iter().fold(0usize, |ac, &size| match size {
    Infinitable::NegInf => unreachable!(),
    Infinitable::Finite(size) => ac.max(size),
    Infinitable::PosInf => ac,
  })
}

const MIN_DIST: usize = 10000;

pub fn p2(s: &str) -> usize {
  let (points, width, height) = parse(s);
  (0..height)
    .flat_map(|y| (0..width).map(move |x| [x, y]))
    .filter(|&p1| {
      let dist: usize = points.iter().map(|&p2| city_dist(p1, p2)).sum();
      dist < MIN_DIST
    })
    .count()
}

#[test]
fn t() {
  let s = include_str!("input/d06.txt");
  assert_eq!(p1(s), 3449);
  assert_eq!(p2(s), 44868);
}
