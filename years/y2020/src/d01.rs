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
  panic!()
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
  panic!()
}

#[test]
fn t() {
  let inp = include_str!("input/d01.txt");
  assert_eq!(p1(inp), 840324);
  assert_eq!(p2(inp), 170098110);
}
