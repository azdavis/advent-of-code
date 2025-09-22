fn parse(s: &str) -> Vec<u32> {
  s.trim().chars().map(|x| x.to_digit(10).unwrap()).collect()
}

pub fn p1(s: &str) -> u32 {
  let digits = parse(s);
  let &first = digits.first().unwrap();
  let &last = digits.last().unwrap();
  digits
    .windows(2)
    .filter(|&w| (w[0] == w[1]))
    .map(|w| w[0])
    .chain((first == last).then_some(first))
    .sum()
}

pub fn p2(s: &str) -> u32 {
  let digits = parse(s);
  let n = digits.len();
  assert_eq!(n % 2, 0);
  let n2 = n / 2;
  let s: u32 = digits
    .iter()
    .take(n2)
    .enumerate()
    .filter_map(|(idx, &a)| (a == digits[idx + n2]).then_some(a))
    .sum();
  s * 2
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 1089);
  assert_eq!(p2(s), 1156);
}
