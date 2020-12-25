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
  let mut cycles: Vec<_> = vec![xs, ys, zs]
    .into_iter()
    .map(|mut moons| {
      let orig = moons.clone();
      let mut ret: usize = 1;
      loop {
        // gravity
        let mut new_moons = moons.clone();
        for i in 0..moons.len() {
          for j in (i + 1)..moons.len() {
            match moons[i].pos.cmp(&moons[j].pos) {
              Ordering::Less => {
                new_moons[i].vel += 1;
                new_moons[j].vel -= 1;
              }
              Ordering::Equal => {}
              Ordering::Greater => {
                new_moons[i].vel -= 1;
                new_moons[j].vel += 1;
              }
            }
          }
        }
        moons = new_moons;
        // velocity
        for m in moons.iter_mut() {
          m.pos += m.vel;
        }
        // check
        if moons == orig {
          return ret;
        }
        ret += 1;
      }
    })
    .collect();
  let fst = cycles.pop().unwrap();
  cycles.into_iter().fold(fst, lcm)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon<T> {
  pos: T,
  vel: T,
}

type Vec3 = [i32; 3];

fn abs_sum(v: Vec3) -> u32 {
  v.iter().map(|n| n.abs() as u32).sum()
}

static RE: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap());

fn p1_go(s: &str, rounds: usize) -> u32 {
  let mut moons: Vec<_> = s
    .lines()
    .map(|line| {
      let cs = RE.captures(line).unwrap();
      Moon {
        pos: [
          cs[1].parse().unwrap(),
          cs[2].parse().unwrap(),
          cs[3].parse().unwrap(),
        ],
        vel: Vec3::default(),
      }
    })
    .collect();
  for _ in 0..rounds {
    // gravity
    let mut new_moons = moons.clone();
    for i in 0..moons.len() {
      for j in (i + 1)..moons.len() {
        for d in 0..3 {
          match moons[i].pos[d].cmp(&moons[j].pos[d]) {
            Ordering::Less => {
              new_moons[i].vel[d] += 1;
              new_moons[j].vel[d] -= 1;
            }
            Ordering::Equal => {}
            Ordering::Greater => {
              new_moons[i].vel[d] -= 1;
              new_moons[j].vel[d] += 1;
            }
          }
        }
      }
    }
    moons = new_moons;
    // velocity
    for m in moons.iter_mut() {
      for d in 0..3 {
        m.pos[d] += m.vel[d];
      }
    }
  }
  moons
    .into_iter()
    .map(|m| abs_sum(m.pos) * abs_sum(m.vel))
    .sum()
}

#[test]
fn t() {
  let inp = include_str!("input/d12.txt");
  assert_eq!(p1(inp), 7928);
  assert_eq!(p2(inp), 518311327635164);
}

#[test]
fn t_p1() {
  let inp = include_str!("input/d12_ex1.txt");
  assert_eq!(p1_go(inp, 10), 179);
  let inp = include_str!("input/d12_ex2.txt");
  assert_eq!(p1_go(inp, 100), 1940);
}

#[test]
fn t_p2() {
  let inp = include_str!("input/d12_ex1.txt");
  assert_eq!(p2(inp), 2772);
  let inp = include_str!("input/d12_ex2.txt");
  assert_eq!(p2(inp), 4686774924);
}
