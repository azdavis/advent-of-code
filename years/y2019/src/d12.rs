use helpers::gcd::lcm;
use helpers::once_cell::sync::Lazy;
use helpers::regex::Regex;
use std::cmp::Ordering;

pub fn p1(s: &str) -> u32 {
  p1_go(s, 1000)
}

/// looked up how to do this on reddit... shameful.
///
/// it seems the key insights are:
/// - the system won't repeat any non-initial state before repeating the initial
///   state. put another way, the first state the system repeats is the initial
///   state.
/// - all the dimensions are independent.
pub fn p2(s: &str) -> usize {
  let [xs, ys, zs] = parse(s);
  lcm(cycle_len(xs), lcm(cycle_len(ys), cycle_len(zs)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
  pos: i32,
  vel: i32,
}

fn cycle_len(mut dim: Vec<Moon>) -> usize {
  let orig = dim.clone();
  let mut ret: usize = 1;
  loop {
    dim = evolve(&dim);
    if dim == orig {
      return ret;
    }
    ret += 1;
  }
}

fn p1_go(s: &str, rounds: usize) -> u32 {
  let mut dims = parse(s);
  for _ in 0..rounds {
    for dim in dims.iter_mut() {
      *dim = evolve(dim);
    }
  }
  let [xs, ys, zs] = dims;
  xs.into_iter()
    .zip(ys)
    .zip(zs)
    .map(|((x, y), z)| {
      let pos = abs(x.pos) + abs(y.pos) + abs(z.pos);
      let vel = abs(x.vel) + abs(y.vel) + abs(z.vel);
      pos * vel
    })
    .sum()
}

fn evolve(dim: &[Moon]) -> Vec<Moon> {
  let mut ret = dim.to_vec();
  // gravity
  for i in 0..dim.len() {
    for j in (i + 1)..dim.len() {
      match dim[i].pos.cmp(&dim[j].pos) {
        Ordering::Less => {
          ret[i].vel += 1;
          ret[j].vel -= 1;
        }
        Ordering::Equal => {}
        Ordering::Greater => {
          ret[i].vel -= 1;
          ret[j].vel += 1;
        }
      }
    }
  }
  // velocity
  for m in ret.iter_mut() {
    m.pos += m.vel;
  }
  ret
}

static RE: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap());

fn parse(s: &str) -> [Vec<Moon>; 3] {
  let mut xs = Vec::new();
  let mut ys = Vec::new();
  let mut zs = Vec::new();
  for line in s.lines() {
    let cs = RE.captures(line).unwrap();
    let x: i32 = cs[1].parse().unwrap();
    let y: i32 = cs[2].parse().unwrap();
    let z: i32 = cs[3].parse().unwrap();
    xs.push(Moon { pos: x, vel: 0 });
    ys.push(Moon { pos: y, vel: 0 });
    zs.push(Moon { pos: z, vel: 0 });
  }
  [xs, ys, zs]
}

fn abs(n: i32) -> u32 {
  n.abs() as u32
}

#[test]
fn t() {
  let s = include_str!("input/d12.txt");
  assert_eq!(p1(s), 7928);
  assert_eq!(p2(s), 518311327635164);
}

#[test]
fn t_p1() {
  let s = include_str!("input/d12_ex1.txt");
  assert_eq!(p1_go(s, 10), 179);
  let s = include_str!("input/d12_ex2.txt");
  assert_eq!(p1_go(s, 100), 1940);
}

#[test]
fn t_p2() {
  let s = include_str!("input/d12_ex1.txt");
  assert_eq!(p2(s), 2772);
  let s = include_str!("input/d12_ex2.txt");
  assert_eq!(p2(s), 4686774924);
}
