use std::collections::HashSet;
use std::convert::TryInto as _;

pub fn p1(s: &str) -> usize {
  let mut map = parse(s);
  for _ in 0..6 {
    map = map
      .iter()
      .flat_map(|&v| {
        let mut ret = neighbors(v);
        ret.push(v);
        ret
      })
      .filter(|&v| {
        let ns_on =
          neighbors(v).into_iter().filter(|n| map.contains(n)).count();
        matches!((map.contains(&v), ns_on), (true, 2) | (_, 3))
      })
      .collect();
  }
  map.len()
}

pub fn p2(s: &str) -> usize {
  todo!()
}

type Vec3 = (i32, i32, i32);

fn neighbors(v: Vec3) -> Vec<Vec3> {
  const LEN: usize = (3 * 3 * 3) - 1;
  let (x, y, z) = v;
  let mut ret = Vec::with_capacity(LEN);
  for dx in -1..=1 {
    for dy in -1..=1 {
      for dz in -1..=1 {
        let add = (x + dx, y + dy, z + dz);
        if add == v {
          continue;
        }
        ret.push(add)
      }
    }
  }
  assert_eq!(ret.len(), LEN);
  ret
}

fn to_i32(n: usize) -> i32 {
  n.try_into().unwrap()
}

fn parse(s: &str) -> HashSet<Vec3> {
  s.split('\n')
    .filter(|line| !line.is_empty())
    .enumerate()
    .flat_map(|(x, line)| {
      line.chars().enumerate().filter_map(move |(y, c)| match c {
        '#' => Some((to_i32(x), to_i32(y), 0)),
        '.' => None,
        _ => panic!("bad char: {}", c),
      })
    })
    .collect()
}

#[test]
fn t() {
  let inp = include_str!("input/d17.txt");
  assert_eq!(p1(inp), 271);
  // assert_eq!(p2(inp), ___);
}
