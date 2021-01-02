pub fn p1(s: &str) -> u32 {
  parse(s).map(|n| n / 3 - 2).sum()
}

pub fn p2(s: &str) -> u32 {
  parse(s).map(p2_one).sum()
}

fn p2_one(mut n: u32) -> u32 {
  let mut acc = 0;
  loop {
    match (n / 3).checked_sub(2) {
      None | Some(0) => return acc,
      Some(x) => {
        acc += x;
        n = x;
      }
    }
  }
}

fn parse(s: &str) -> impl Iterator<Item = u32> + '_ {
  s.lines().map(|line| line.parse().unwrap())
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 3296560);
  assert_eq!(p2(s), 4941976);
}
