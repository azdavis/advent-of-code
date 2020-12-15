pub fn p1(s: &str) -> usize {
  let inp = parse(s);
  let start = inp.start;
  let (wait, bus) = inp
    .buses
    .into_iter()
    .map(|bus| (wait_time(start, bus.id), bus))
    .min_by_key(|x| x.0)
    .unwrap();
  bus.id * wait
}

pub fn p2(s: &str) -> usize {
  let buses = parse(s).buses;
  let mut iter = buses.into_iter();
  let fst = iter.next().unwrap();
  assert_eq!(fst.idx, 0);
  let mut overall_lcm = fst.id;
  let mut sum = 0;
  for bus in iter {
    let off = wait_time(sum + bus.idx, bus.id);
    let delta = get_delta(overall_lcm, off, bus.id);
    sum += delta;
    overall_lcm = lcm(overall_lcm, bus.id);
  }
  sum
}

/// returns the smallest number that is both:
/// 1. an exact multiple of `a`
/// 2. `off` more than a multiple of `b`
fn get_delta(a: usize, off: usize, b: usize) -> usize {
  let mut idx = 0;
  loop {
    let ret = a * idx;
    if ret.checked_sub(off).map_or(false, |y| y % b == 0) {
      return ret;
    }
    idx += 1;
  }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
  assert!(a != 0 || b != 0);
  while b != 0 {
    let tmp = b;
    b = a % b;
    a = tmp;
  }
  a
}

fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

fn wait_time(start: usize, id: usize) -> usize {
  let r = start % id;
  if r == 0 {
    0
  } else {
    id - r
  }
}

struct Input {
  start: usize,
  buses: Vec<Bus>,
}

struct Bus {
  idx: usize,
  id: usize,
}

fn parse(s: &str) -> Input {
  let mut lines = s.split('\n');
  Input {
    start: lines.next().unwrap().parse().unwrap(),
    buses: lines
      .next()
      .unwrap()
      .split(',')
      .enumerate()
      .filter(|&(_, x)| x != "x")
      .map(|(idx, x)| Bus {
        idx,
        id: x.parse().unwrap(),
      })
      .collect(),
  }
}

#[test]
fn t() {
  let inp = include_str!("input/d13.txt");
  assert_eq!(p1(inp), 333);
  assert_eq!(p2(inp), 690123192779524);
}
