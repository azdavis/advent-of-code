const TARGET: u32 = 2020;

fn parse(s: &str) -> Vec<u32> {
  s.split_ascii_whitespace()
    .map(str::parse)
    .collect::<Result<_, _>>()
    .unwrap()
}

pub fn p1(s: &str) {
  let xs = parse(s);
  for &a in xs.iter() {
    for &b in xs.iter() {
      if a + b == TARGET {
        println!("{}", a * b);
        return;
      }
    }
  }
}

pub fn p2(s: &str) {
  let xs = parse(s);
  for &a in xs.iter() {
    for &b in xs.iter() {
      for &c in xs.iter() {
        if a + b + c == TARGET {
          println!("{}", a * b * c);
          return;
        }
      }
    }
  }
}
