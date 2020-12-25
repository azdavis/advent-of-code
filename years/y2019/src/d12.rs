use helpers::gcd::lcm;
use helpers::once_cell::sync::Lazy;
use helpers::regex::Regex;
use std::cmp::Ordering;

pub fn p1(s: &str) -> u32 {
  p1_go(s, 1000)
}

pub fn p2(s: &str) -> u32 {
  todo!()
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
  // assert_eq!(p2(inp), ___);
}

#[test]
fn t_p1() {
  let inp = include_str!("input/d12_ex1.txt");
  assert_eq!(p1_go(inp, 10), 179);
  let inp = include_str!("input/d12_ex2.txt");
  assert_eq!(p1_go(inp, 100), 1940);
}
