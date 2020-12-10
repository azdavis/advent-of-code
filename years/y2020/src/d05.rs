pub fn p1(s: &str) {
  let ans = s
    .split('\n')
    .filter_map(|s| {
      if s.is_empty() {
        None
      } else {
        Some(seat_id(parse_seat(s)))
      }
    })
    .max()
    .unwrap();
  println!("{}", ans);
}

struct Seat {
  row: u32,
  col: u32,
}

fn parse_seat(s: &str) -> Seat {
  let bs = s.as_bytes();
  let mut idx = 0;
  let mut lo = 0;
  let mut hi = 127;
  for _ in 0..7 {
    let mid = (lo + hi) / 2;
    match bs[idx] {
      b'F' => hi = mid,
      b'B' => lo = mid + 1,
      _ => unreachable!(),
    }
    idx += 1;
  }
  assert_eq!(lo, hi);
  let row = lo;
  lo = 0;
  hi = 7;
  for _ in 0..3 {
    let mid = (lo + hi) / 2;
    match bs[idx] {
      b'L' => hi = mid,
      b'R' => lo = mid + 1,
      _ => unreachable!(),
    }
    idx += 1;
  }
  assert_eq!(lo, hi);
  Seat { row, col: lo }
}

fn seat_id(s: Seat) -> u32 {
  s.row * 8 + s.col
}
