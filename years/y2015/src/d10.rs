use helpers::digits;

fn digits_u8(n: u32) -> impl Iterator<Item = u8> {
  digits::get(n).map(|x| x.try_into().unwrap())
}

fn run(n: u32, rounds: usize) -> Vec<u8> {
  let mut cur: Vec<u8> = digits_u8(n).collect();
  for _ in 0..rounds {
    let mut next = Vec::with_capacity(cur.capacity());
    let mut iter = cur.into_iter();
    let mut run_digit = iter.next().unwrap();
    let mut run_len = 1u32;
    for digit in iter {
      if digit == run_digit {
        run_len += 1;
      } else {
        next.extend(digits_u8(run_len));
        next.push(run_digit);
        run_digit = digit;
        run_len = 1;
      }
    }
    next.extend(digits_u8(run_len));
    next.push(run_digit);
    cur = next;
  }
  cur
}

fn parse(s: &str) -> u32 {
  s.trim().parse().unwrap()
}

pub fn p1(s: &str) -> usize {
  run(parse(s), 40).len()
}

pub fn p2(s: &str) -> usize {
  run(parse(s), 50).len()
}

#[test]
fn t() {
  let s = include_str!("input/d10.txt");
  assert_eq!(p1(s), 492982);
  assert_eq!(p2(s), 6989950);
}

#[test]
fn ex() {
  assert_eq!(run(1, 1), [1, 1]);
  assert_eq!(run(11, 1), [2, 1]);
  assert_eq!(run(1, 2), [2, 1]);
  assert_eq!(run(21, 1), [1, 2, 1, 1]);
  assert_eq!(run(1, 3), [1, 2, 1, 1]);
  assert_eq!(run(1211, 1), [1, 1, 1, 2, 2, 1]);
  assert_eq!(run(1, 4), [1, 1, 1, 2, 2, 1]);
  assert_eq!(run(111221, 1), [3, 1, 2, 2, 1, 1]);
  assert_eq!(run(1, 5), [3, 1, 2, 2, 1, 1]);
}
