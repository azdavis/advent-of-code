fn run(s: &str, f: fn(Vec<u32>) -> u32) -> u32 {
  s.lines()
    .map(|line| {
      let ns: Vec<u32> = line
        .split_ascii_whitespace()
        .map(|it| it.parse().unwrap())
        .collect();
      f(ns)
    })
    .sum()
}

pub fn p1(s: &str) -> u32 {
  run(s, |ns| ns.iter().max().unwrap() - ns.iter().min().unwrap())
}

pub fn p2(s: &str) -> u32 {
  run(s, |ns| {
    for a in &ns {
      for b in &ns {
        if a != b && (a % b == 0 || b % a == 0) {
          return a.max(b) / a.min(b);
        }
      }
    }
    panic!("no solution for {ns:?}")
  })
}

#[test]
fn t() {
  let s = include_str!("input/d02.txt");
  assert_eq!(p1(s), 53460);
  assert_eq!(p2(s), 282);
}
