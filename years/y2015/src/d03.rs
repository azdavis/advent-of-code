use helpers::hash_set;

fn parse(s: &str) -> impl Iterator<Item = [i32; 2]> + '_ {
  s.trim().chars().map(|c| match c {
    '^' => [0, 1],
    'v' => [0, -1],
    '<' => [-1, 0],
    '>' => [1, 0],
    _ => panic!("unknown char: {}", c),
  })
}

pub fn p1(s: &str) -> usize {
  let mut x = 0i32;
  let mut y = 0i32;
  let mut set = hash_set([[x, y]]);
  for [dx, dy] in parse(s) {
    x += dx;
    y += dy;
    set.insert([x, y]);
  }
  set.len()
}

pub fn p2(s: &str) -> usize {
  let mut x1 = 0i32;
  let mut y1 = 0i32;
  let mut x2 = 0i32;
  let mut y2 = 0i32;
  let mut set = hash_set([[x1, y1]]);
  for (idx, [dx, dy]) in parse(s).enumerate() {
    if idx % 2 == 0 {
      x1 += dx;
      y1 += dy;
      set.insert([x1, y1]);
    } else {
      x2 += dx;
      y2 += dy;
      set.insert([x2, y2]);
    }
  }
  set.len()
}

#[test]
fn t() {
  let s = include_str!("input/d03.txt");
  assert_eq!(p1(s), 2592);
  assert_eq!(p2(s), 2360);
}
