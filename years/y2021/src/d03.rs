use std::cmp::Ordering;

fn byte_to_digit(b: u8) -> usize {
  match b {
    b'0' => 0,
    b'1' => 1,
    _ => panic!("unknown byte: {}", b),
  }
}

fn get_digit(n1: usize, total: usize, ord: Ordering) -> usize {
  let n0 = total - n1;
  match (n1.cmp(&n0), ord) {
    (_, Ordering::Equal) => unreachable!(),
    (Ordering::Less, Ordering::Less)
    | (Ordering::Greater, Ordering::Greater)
    | (Ordering::Equal, Ordering::Greater) => 1,
    (Ordering::Less, Ordering::Greater)
    | (Ordering::Greater, Ordering::Less)
    | (Ordering::Equal, Ordering::Less) => 0,
  }
}

fn mk_num<I>(iter: I) -> usize
where
  I: Iterator<Item = usize>,
{
  iter.fold(0usize, |ac, d| (ac << 1) | d)
}

pub fn p1(s: &str) -> usize {
  let mut lines = s.lines();
  let mut ones: Vec<_> =
    lines.next().unwrap().bytes().map(byte_to_digit).collect();
  let mut n = 1usize;
  for line in lines {
    assert_eq!(ones.len(), line.len());
    for (x, c) in ones.iter_mut().zip(line.bytes()) {
      *x += byte_to_digit(c);
    }
    n += 1;
  }
  let gam = mk_num(ones.iter().map(|&n1| get_digit(n1, n, Ordering::Greater)));
  let eps = mk_num(ones.iter().map(|&n1| get_digit(n1, n, Ordering::Less)));
  gam * eps
}

fn p2_help(mut ac: Vec<Vec<usize>>, ord: Ordering) -> usize {
  let n = ac.first().unwrap().len();
  for i in 0..n {
    assert!(!ac.is_empty());
    let n1: usize = ac.iter().map(|line| line[i]).sum();
    let want = get_digit(n1, ac.len(), ord);
    ac.retain(|line| line[i] == want);
    if ac.len() == 1 {
      return mk_num(ac.pop().unwrap().into_iter());
    }
  }
  panic!("no solution")
}

pub fn p2(s: &str) -> usize {
  let lines: Vec<Vec<_>> = s
    .lines()
    .map(|s| s.bytes().map(byte_to_digit).collect())
    .collect();
  let oxy = p2_help(lines.clone(), Ordering::Greater);
  let co2 = p2_help(lines, Ordering::Less);
  oxy * co2
}

#[test]
fn t() {
  let s = include_str!("input/d03.txt");
  assert_eq!(p1(s), 3549854);
  assert_eq!(p2(s), 3765399);
}

#[test]
fn ex1() {
  let s = include_str!("input/d03_ex1.txt");
  assert_eq!(p1(s), 198);
  assert_eq!(p2(s), 230);
}
