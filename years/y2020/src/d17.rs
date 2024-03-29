use helpers::HashSet;
use std::hash::Hash;

pub fn p1(s: &str) -> usize {
  let inp: HashSet<_> = parse(s).map(|(a, b)| (a, b, 0)).collect();
  go(inp, neighbors_vec3)
}

pub fn p2(s: &str) -> usize {
  let inp: HashSet<_> = parse(s).map(|(a, b)| (a, b, 0, 0)).collect();
  go(inp, neighbors_vec4)
}

fn go<T>(mut set: HashSet<T>, neighbors: fn(T) -> Vec<T>) -> usize
where
  T: Hash + Eq + Copy,
{
  for _ in 0..6 {
    set = set
      .iter()
      .flat_map(|&v| {
        let mut ns = neighbors(v);
        ns.push(v);
        ns
      })
      .filter(|&v| {
        let ns_on = neighbors(v).into_iter().filter(|n| set.contains(n)).count();
        matches!((set.contains(&v), ns_on), (true, 2) | (_, 3))
      })
      .collect();
  }
  set.len()
}

type Vec3 = (i32, i32, i32);

fn neighbors_vec3(v: Vec3) -> Vec<Vec3> {
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
        ret.push(add);
      }
    }
  }
  assert_eq!(ret.len(), LEN);
  ret
}

type Vec4 = (i32, i32, i32, i32);

#[allow(clippy::many_single_char_names)]
fn neighbors_vec4(v: Vec4) -> Vec<Vec4> {
  const LEN: usize = (3 * 3 * 3 * 3) - 1;
  let (w, x, y, z) = v;
  let mut ret = Vec::with_capacity(LEN);
  for dw in -1..=1 {
    for dx in -1..=1 {
      for dy in -1..=1 {
        for dz in -1..=1 {
          let add = (w + dw, x + dx, y + dy, z + dz);
          if add == v {
            continue;
          }
          ret.push(add);
        }
      }
    }
  }
  assert_eq!(ret.len(), LEN);
  ret
}

fn to_i32(n: usize) -> i32 {
  n.try_into().unwrap()
}

fn parse(s: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
  s.lines().enumerate().flat_map(|(x, line)| {
    line.chars().enumerate().filter_map(move |(y, c)| match c {
      '#' => Some((to_i32(x), to_i32(y))),
      '.' => None,
      _ => panic!("bad char: {c}"),
    })
  })
}

#[test]
fn t() {
  let s = include_str!("input/d17.txt");
  assert_eq!(p1(s), 271);
  assert_eq!(p2(s), 2064);
}
