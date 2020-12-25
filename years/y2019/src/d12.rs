use helpers::once_cell::sync::Lazy;
use helpers::regex::Regex;
use std::cmp::Ordering;

pub fn p1(s: &str) -> u32 {
  p1_go(s, 1000)
}

pub fn p2(s: &str) -> u32 {
  todo!()
}

type Vec3 = [i32; 3];

fn abs_sum(v: Vec3) -> u32 {
  v.iter().map(|n| n.abs() as u32).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon<T> {
  pos: T,
  vel: T,
}

macro_rules! update_vel {
  ($old: ident, $new: ident, $i: ident, $j: ident, $dim: expr) => {
    match $old[$i].pos[$dim].cmp(&$old[$j].pos[$dim]) {
      Ordering::Less => {
        $new[$i].vel[$dim] += 1;
        $new[$j].vel[$dim] -= 1;
      }
      Ordering::Equal => {}
      Ordering::Greater => {
        $new[$i].vel[$dim] -= 1;
        $new[$j].vel[$dim] += 1;
      }
    }
  };
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
        update_vel!(moons, new_moons, i, j, 0);
        update_vel!(moons, new_moons, i, j, 1);
        update_vel!(moons, new_moons, i, j, 2);
      }
    }
    moons = new_moons;
    // velocity
    for m in moons.iter_mut() {
      m.pos[0] += m.vel[0];
      m.pos[1] += m.vel[1];
      m.pos[2] += m.vel[2];
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
fn t_ex() {
  let inp = include_str!("input/d12_ex1.txt");
  assert_eq!(p1_go(inp, 10), 179);
  let inp = include_str!("input/d12_ex2.txt");
  assert_eq!(p1_go(inp, 100), 1940);
}
