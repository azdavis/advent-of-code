use std::ops::RangeInclusive;

fn parse_range(s: &str) -> RangeInclusive<u16> {
  let (fst, snd) = s.split_once('-').unwrap();
  fst.parse().unwrap()..=snd.parse().unwrap()
}

fn range_contains<T, F>(fst: &RangeInclusive<T>, snd: &RangeInclusive<T>, f: &mut F) -> bool
where
  T: PartialOrd,
  F: FnMut(bool, bool) -> bool,
{
  f(fst.contains(snd.start()), fst.contains(snd.end()))
}

fn go<F>(s: &str, f: &mut F) -> usize
where
  F: FnMut(bool, bool) -> bool,
{
  s.lines()
    .filter(|line| {
      let (fst, snd) = line.split_once(',').unwrap();
      let fst = parse_range(fst);
      let snd = parse_range(snd);
      range_contains(&fst, &snd, f) || range_contains(&snd, &fst, f)
    })
    .count()
}

pub fn p1(s: &str) -> usize {
  go(s, &mut |a, b| a && b)
}

pub fn p2(s: &str) -> usize {
  go(s, &mut |a, b| a || b)
}

#[test]
fn t() {
  let s = include_str!("input/d04.txt");
  assert_eq!(p1(s), 441);
  assert_eq!(p2(s), 861);
}
