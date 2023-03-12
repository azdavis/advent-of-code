use helpers::{hash_map, HashMap};

type Coord = [u8; 2];

fn run(s: &str, map: &HashMap<Coord, char>, mut cur: Coord) -> String {
  let mut ret = String::new();
  for line in s.lines() {
    for c in line.chars() {
      let [x, y] = cur;
      let next = match c {
        'U' => match y.checked_sub(1) {
          None => continue,
          Some(y) => [x, y],
        },
        'D' => [x, y + 1],
        'L' => match x.checked_sub(1) {
          None => continue,
          Some(x) => [x, y],
        },
        'R' => [x + 1, y],
        _ => panic!("unknown char: {c}"),
      };
      if map.contains_key(&next) {
        cur = next;
      }
    }
    ret.push(map[&cur]);
  }
  ret
}

pub fn p1(s: &str) -> String {
  let map = hash_map([
    // 1 2 3
    ([0, 0], '1'),
    ([1, 0], '2'),
    ([2, 0], '3'),
    // 4 5 6
    ([0, 1], '4'),
    ([1, 1], '5'),
    ([2, 1], '6'),
    // 7 8 9
    ([0, 2], '7'),
    ([1, 2], '8'),
    ([2, 2], '9'),
  ]);
  run(s, &map, [1, 1])
}

pub fn p2(s: &str) -> String {
  let map = hash_map([
    //     1
    ([2, 0], '1'),
    //   2 3 4
    ([1, 1], '2'),
    ([2, 1], '3'),
    ([3, 1], '4'),
    // 5 6 7 8 9
    ([0, 2], '5'),
    ([1, 2], '6'),
    ([2, 2], '7'),
    ([3, 2], '8'),
    ([4, 2], '9'),
    //   A B C
    ([1, 3], 'A'),
    ([2, 3], 'B'),
    ([3, 3], 'C'),
    //     D
    ([2, 4], 'D'),
  ]);
  run(s, &map, [0, 2])
}

#[test]
fn t() {
  let s = include_str!("input/d02.txt");
  assert_eq!(p1(s), "99332");
  assert_eq!(p2(s), "DD483");
}

#[test]
fn ex1() {
  let s = include_str!("input/d02_ex1.txt");
  assert_eq!(p1(s), "1985");
  assert_eq!(p2(s), "5DB3");
}
