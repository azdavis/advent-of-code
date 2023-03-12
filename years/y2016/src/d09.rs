use helpers::Counter;

fn expand<'s>(mut s: &'s str, overall_count: usize, map: &mut Counter<&'s str>) {
  while let Some((before, after)) = s.split_once('(') {
    let (sigil, rest) = after.split_once(')').unwrap();
    let (len, count) = sigil.split_once('x').unwrap();
    let len: usize = len.parse().unwrap();
    let count: usize = count.parse().unwrap();
    let repeated = &rest[..len];
    s = &rest[len..];
    map.add(before, overall_count);
    map.add(repeated, overall_count * count);
  }
  map.add(s, overall_count);
}

fn sum_all(map: Counter<&str>) -> usize {
  map.into_iter().map(|(k, v)| k.len() * v).sum()
}

pub fn p1(s: &str) -> usize {
  let mut map = Counter::<&str>::default();
  expand(s.trim(), 1, &mut map);
  sum_all(map)
}

pub fn p2(s: &str) -> usize {
  let mut map = Counter::<&str>::default();
  expand(s.trim(), 1, &mut map);
  loop {
    let mut new_map = Counter::<&str>::with_capacity(map.len());
    for (&k, count) in map.iter() {
      expand(k, count, &mut new_map);
    }
    if map == new_map {
      break;
    }
    map = new_map;
  }
  sum_all(map)
}

#[test]
fn t() {
  let s = include_str!("input/d09.txt");
  assert_eq!(p1(s), 97714);
  assert_eq!(p2(s), 10_762_972_461);
}
