use std::collections::HashSet;

pub fn p1(s: &str) -> usize {
  parse(s).len()
}

pub fn p2(s: &str) -> usize {
  let mut set = parse(s);
  for _ in 0..100 {
    let to_consider: HashSet<_> =
      set.iter().flat_map(|&c| Vec::from(neighbors(c))).collect();
    set = to_consider
      .into_iter()
      .filter(|&c| {
        let n = neighbors(c).iter().filter(|n| set.contains(n)).count();
        if set.contains(&c) {
          matches!(n, 1 | 2)
        } else {
          n == 2
        }
      })
      .collect();
  }
  set.len()
}

/// https://www.redblobgames.com/grids/hexagons/#coordinates-axial
type AxialCoord = (i32, i32);

fn neighbors(c: AxialCoord) -> [AxialCoord; 6] {
  let (q, r) = c;
  [
    (q, r - 1),
    (q + 1, r - 1),
    (q + 1, r),
    (q, r + 1),
    (q - 1, r + 1),
    (q - 1, r),
  ]
}

fn parse(s: &str) -> HashSet<AxialCoord> {
  let mut ret = HashSet::new();
  for line in s.lines() {
    let coord = parse_axial_coord(line);
    if ret.contains(&coord) {
      assert!(ret.remove(&coord));
    } else {
      assert!(ret.insert(coord));
    }
  }
  ret
}

fn parse_axial_coord(s: &str) -> AxialCoord {
  let mut q = 0;
  let mut r = 0;
  let mut chars = s.chars();
  while let Some(c) = chars.next() {
    match c {
      'n' => {
        r -= 1;
        match chars.next().unwrap() {
          'w' => {}
          'e' => q += 1,
          bad => panic!("bad char after n: {}", bad),
        }
      }
      's' => {
        r += 1;
        match chars.next().unwrap() {
          'w' => q -= 1,
          'e' => {}
          bad => panic!("bad char after s: {}", bad),
        }
      }
      'w' => q -= 1,
      'e' => q += 1,
      _ => panic!("bad char: {}", c),
    }
  }
  (q, r)
}

#[test]
fn t() {
  let inp = include_str!("input/d24.txt");
  assert_eq!(p1(inp), 497);
  assert_eq!(p2(inp), 4156);
}
