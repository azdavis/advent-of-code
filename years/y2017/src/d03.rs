mod spiral;

use helpers::neighbors::{self, SignedCoord};
use helpers::HashMap;

pub fn p1(s: &str) -> u32 {
  let idx: usize = s.trim().parse().unwrap();
  let [x, y] = spiral::Spiral::default().nth(idx - 1).unwrap();
  (x.abs() + y.abs()).try_into().unwrap()
}

pub fn p2(s: &str) -> u32 {
  let n: u32 = s.trim().parse().unwrap();
  let mut map = HashMap::<SignedCoord, u32>::default();
  let mut iter = spiral::Spiral::default();
  map.insert(iter.next().unwrap(), 1);
  for coord in iter {
    let val: u32 = neighbors::signed(coord)
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
  assert_eq!(p2(s), 330_785);
}
