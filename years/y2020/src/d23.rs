use helpers::digits::to_char;

pub fn p1(s: &str) -> String {
  go_p1(s, 100)
}

pub fn p2(s: &str) -> u64 {
  let (mut cups, first) = mk_map_p2(s);
  go(&mut cups, first, 10_000_000);
  let n1 = cups[1];
  let n2 = cups[to_usize(n1)];
  u64::from(n1) * u64::from(n2)
}

fn go_p1(s: &str, rounds: usize) -> String {
  let (mut cups, first) = mk_map_p1(s);
  go(&mut cups, first, rounds);
  let mut ret = String::with_capacity(cups.len() - 1);
  let mut cur = cups[1];
  while cur != 1 {
    ret.push(to_char(cur));
    cur = cups[to_usize(cur)];
  }
  ret
}

fn mk_map_p1(s: &str) -> (Vec<u32>, u32) {
  let cups = parse(s);
  let first = *cups.first().unwrap();
  let last = *cups.last().unwrap();
  let max = *cups.iter().max().unwrap();
  let mut m = vec![0; to_usize(max + 1)];
  for xs in cups.windows(2) {
    m[to_usize(xs[0])] = xs[1];
  }
  m[to_usize(last)] = first;
  (m, first)
}

const MAX_ELEM_P2: u32 = 1_000_000;

fn mk_map_p2(s: &str) -> (Vec<u32>, u32) {
  let cups = parse(s);
  let first = *cups.first().unwrap();
  let last = *cups.last().unwrap();
  let max = *cups.iter().max().unwrap();
  let mut m = vec![0; to_usize(MAX_ELEM_P2 + 1)];
  for xs in cups.windows(2) {
    m[to_usize(xs[0])] = xs[1];
  }
  m[to_usize(last)] = max + 1;
  for x in max + 1..MAX_ELEM_P2 {
    m[to_usize(x)] = x + 1;
  }
  m[to_usize(MAX_ELEM_P2)] = first;
  (m, first)
}

// take advantage of the fact that there are no duplicate elements to avoid both
// indexing logic and linked lists, and just map each element to its successor.
// but use a Vec as the map with vec[0] = 0 since we also know there won't be
// a whole lot of wasted space.
fn go(cups: &mut Vec<u32>, mut cur: u32, rounds: usize) {
  assert!(cups.len() >= 5);
  let min_cup = to_u32(cups.iter().position(|&c| c != 0).unwrap());
  let max_cup = to_u32(cups.len() - 1);
  for _ in 0..rounds {
    let pick_up: Vec<_> = (0..3)
      .map(|_| {
        let to_rm = cups[to_usize(cur)];
        let next = cups[to_usize(to_rm)];
        cups[to_usize(cur)] = next;
        to_rm
      })
      .collect();
    let mut dest = cur - 1;
    loop {
      if dest < min_cup {
        dest = max_cup;
      }
      if !pick_up.contains(&dest) {
        break;
      }
      dest -= 1;
    }
    let dest = to_usize(dest);
    for n in pick_up.into_iter().rev() {
      let next = cups[dest];
      cups[dest] = n;
      cups[to_usize(n)] = next;
    }
    cur = cups[to_usize(cur)];
  }
}

fn parse(s: &str) -> Vec<u32> {
  s.trim_end()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect()
}

fn to_usize(n: u32) -> usize {
  n.try_into().unwrap()
}

fn to_u32(n: usize) -> u32 {
  n.try_into().unwrap()
}

#[test]
fn t() {
  let s = include_str!("input/d23.txt");
  assert_eq!(p1(s), "89372645");
  assert_eq!(p2(s), 21_273_394_210);
}

#[cfg(test)]
mod tests {
  use super::{go_p1, mk_map_p1, mk_map_p2, p2, to_usize, MAX_ELEM_P2};

  #[test]
  fn t_p1() {
    assert_eq!(go_p1("389125467", 10), "92658374");
    assert_eq!(go_p1("389125467", 100), "67384529");
  }

  #[test]
  fn t_p2() {
    assert_eq!(p2("389125467"), 149_245_887_792);
  }

  #[test]
  fn t_mk_map_p1() {
    let (map, fst) = mk_map_p1("45312");
    let want = vec![0, 2, 4, 1, 5, 3];
    assert_eq!(map, want);
    assert_eq!(fst, 4);
  }

  #[test]
  fn t_mk_map_p2() {
    let (map, fst) = mk_map_p2("3215674");
    assert_eq!(map.len(), to_usize(MAX_ELEM_P2 + 1));
    assert_eq!(map[3], 2);
    assert_eq!(map[2], 1);
    assert_eq!(map[1], 5);
    assert_eq!(map[5], 6);
    assert_eq!(map[6], 7);
    assert_eq!(map[7], 4);
    assert_eq!(map[4], 8);
    for i in 8..MAX_ELEM_P2 {
      assert_eq!(map[to_usize(i)], i + 1);
    }
    assert_eq!(map[to_usize(MAX_ELEM_P2)], 3);
    assert_eq!(fst, 3);
  }
}
