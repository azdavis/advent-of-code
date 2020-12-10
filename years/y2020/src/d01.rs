const TARGET: u32 = 2020;

fn parse(s: &str) -> Vec<u32> {
  s.split_ascii_whitespace()
    .map(str::parse)
    .collect::<Result<_, _>>()
    .unwrap()
}

pub fn p1(s: &str) -> u32 {
  let xs = parse(s);
  for &a in xs.iter() {
    for &b in xs.iter() {
      if a + b == TARGET {
        return a * b;
      }
    }
  }
  unreachable!()
}

pub fn p2(s: &str) -> u32 {
  let xs = parse(s);
  for &a in xs.iter() {
    for &b in xs.iter() {
      for &c in xs.iter() {
        if a + b + c == TARGET {
          return a * b * c;
        }
      }
    }
  }
  unreachable!()
}
