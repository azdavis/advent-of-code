pub fn p1(s: &str) -> u32 {
  let xs = parse(s);
  for &a in &xs {
    for &b in &xs {
      if a + b == TARGET {
        return a * b;
      }
    }
  }
  panic!("no solution")
}

pub fn p2(s: &str) -> u32 {
  let xs = parse(s);
  for &a in &xs {
    for &b in &xs {
      for &c in &xs {
        if a + b + c == TARGET {
          return a * b * c;
        }
      }
    }
  }
  panic!("no solution")
}

const TARGET: u32 = 2020;

fn parse(s: &str) -> Vec<u32> {
  s.lines().map(str::parse).collect::<Result<_, _>>().unwrap()
}

#[test]
fn t() {
  let s = include_str!("input/d01.txt");
  assert_eq!(p1(s), 840_324);
  assert_eq!(p2(s), 170_098_110);
}
