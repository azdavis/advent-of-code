fn parse(s: &str) -> impl Iterator<Item = u16> + '_ {
  s.lines().map(|x| x.parse().unwrap())
}

fn count_increased<I>(iter: I) -> usize
where
  I: Iterator<Item = u16>,
{
  let mut prev = None::<u16>;
  iter
    .filter(|&x| {
      let ret = prev.map_or(false, |y| y < x);
      prev = Some(x);
      ret
    })
    .count()
}

pub fn p1(s: &str) -> usize {
  count_increased(parse(s))
}

pub fn p2(s: &str) -> usize {
  let ns: Vec<_> = parse(s).collect();
  count_increased(ns.windows(3).map(|w| w.iter().sum()))
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 1557);
  assert_eq!(p2(s), 1608);
}
