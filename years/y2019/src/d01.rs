pub fn p1(s: &str) -> u32 {
  parse(s).map(|n| n / 3 - 2).sum()
}

pub fn p2(_: &str) -> u32 {
  todo!()
}

fn parse(s: &str) -> impl Iterator<Item = u32> + '_ {
  s.split('\n')
    .filter(|line| !line.is_empty())
    .map(|line| line.parse().unwrap())
}
