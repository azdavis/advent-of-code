mod spiral;

use helpers::HashMap;

pub fn p1(s: &str) -> u32 {
  let idx: usize = s.trim().parse().unwrap();
  let [x, y] = spiral::Spiral::default().nth(idx - 1).unwrap();
  (x.abs() + y.abs()).try_into().unwrap()
}

type Coord = [i32; 2];

fn neighbors(coord: Coord) -> [Coord; 8] {
  let [x, y] = coord;
  [
    [x - 1, y - 1],
    [x, y - 1],
    [x + 1, y - 1],
    [x - 1, y],
    [x + 1, y],
    [x - 1, y + 1],
    [x, y + 1],
    [x + 1, y + 1],
  ]
}

pub fn p2(s: &str) -> u32 {
  let n: u32 = s.trim().parse().unwrap();
  let mut map = HashMap::<Coord, u32>::default();
  let mut iter = spiral::Spiral::default();
  map.insert(iter.next().unwrap(), 1);
  for coord in iter {
    let val: u32 = neighbors(coord)
      .into_iter()
      .filter_map(|ne| map.get(&ne))
      .sum();
    if val > n {
      return val;
    }
    map.insert(coord, val);
  }
  unreachable!()
}

#[test]
fn t() {
  let s = include_str!("input/d03.txt");
  assert_eq!(p1(s), 552);
  assert_eq!(p2(s), 330785);
}
