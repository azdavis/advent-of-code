fn parse(s: &str) -> Vec<u32> {
  s.trim().split(',').map(|x| x.parse().unwrap()).collect()
}

fn diff(a: u32, b: u32) -> u32 {
  a.abs_diff(b)
}

fn run(s: &str, cost: fn(u32, u32) -> u32) -> u32 {
  let xs = parse(s);
  let min = xs.iter().copied().min().unwrap();
  let max = xs.iter().copied().max().unwrap();
  (min..=max)
    .map(|a| xs.iter().map(|&b| cost(a, b)).sum::<u32>())
    .min()
    .unwrap()
}

pub fn p1(s: &str) -> u32 {
  run(s, diff)
}

pub fn p2(s: &str) -> u32 {
  run(s, |a, b| {
    let n = diff(a, b);
    ((n + 1) * n) / 2
  })
}

#[test]
fn t() {
  let s = include_str!("input/d07.txt");
  assert_eq!(p1(s), 344_605);
  assert_eq!(p2(s), 93_699_985);
}
