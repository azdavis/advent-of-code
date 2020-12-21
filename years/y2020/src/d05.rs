use std::collections::HashSet;

pub fn p1(s: &str) -> u32 {
  seat_ids(s).max().unwrap()
}

pub fn p2(s: &str) -> u32 {
  let all_ids: HashSet<_> = seat_ids(s).collect();
  let max_possible = seat_id(Seat {
    row: MAX_ROW,
    col: MAX_COL,
  });
  for id in 0..=max_possible {
    if all_ids.contains(&id) {
      continue;
    }
    let id_less_1 = match id.checked_sub(1) {
      None => continue,
      Some(x) => x,
    };
    if all_ids.contains(&id_less_1) && all_ids.contains(&(id + 1)) {
      return id;
    }
  }
  panic!("no solution")
}

fn seat_ids(s: &str) -> impl Iterator<Item = u32> + '_ {
  s.split('\n').filter_map(|s| {
    if s.is_empty() {
      None
    } else {
      Some(seat_id(parse_seat(s)))
    }
  })
}

const MAX_ROW: u32 = 127;
const MAX_COL: u32 = 7;

struct Seat {
  row: u32,
  col: u32,
}

fn parse_seat(s: &str) -> Seat {
  let bs = s.as_bytes();
  let mut idx = 0;
  let mut lo = 0;
  let mut hi = MAX_ROW;
  for _ in 0..7 {
    let mid = (lo + hi) / 2;
    match bs[idx] {
      b'F' => hi = mid,
      b'B' => lo = mid + 1,
      seat => panic!("bad seat: {}", seat),
    }
    idx += 1;
  }
  assert_eq!(lo, hi);
  let row = lo;
  lo = 0;
  hi = MAX_COL;
  for _ in 0..3 {
    let mid = (lo + hi) / 2;
    match bs[idx] {
      b'L' => hi = mid,
      b'R' => lo = mid + 1,
      seat => panic!("bad seat: {}", seat),
    }
    idx += 1;
  }
  assert_eq!(lo, hi);
  Seat { row, col: lo }
}

fn seat_id(s: Seat) -> u32 {
  s.row * (MAX_COL + 1) + s.col
}

#[test]
fn t() {
  let inp = include_str!("input/d05.txt");
  assert_eq!(p1(inp), 861);
  assert_eq!(p2(inp), 633);
}
