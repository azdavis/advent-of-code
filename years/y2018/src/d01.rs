use helpers::HashSet;

fn parse(s: &str) -> impl Iterator<Item = i32> + '_ {
  s.lines().map(|line| line.parse::<i32>().unwrap())
}

pub fn p1(s: &str) -> i32 {
  parse(s).sum()
}

pub fn p2(s: &str) -> i32 {
  let ns: Vec<_> = parse(s).collect();
  let mut cur = 0i32;
  let mut past = HashSet::<i32>::default();
  loop {
    for &n in ns.iter() {
      cur += n;
      if !past.insert(cur) {
        return cur;
      }
    }
  }
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 516);
  assert_eq!(p2(s), 71892);
}
