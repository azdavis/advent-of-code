use helpers::regex::Regex;

#[allow(clippy::naive_bytecount)]
pub fn p1(s: &str) -> usize {
  go(s, |n1, n2, c, pw| {
    let count = pw.as_bytes().iter().filter(|&&b| b == c).count();
    n1 <= count && count <= n2
  })
}

pub fn p2(s: &str) -> usize {
  go(s, |n1, n2, c, pw| {
    let bs = pw.as_bytes();
    let at_n1 = bs[n1 - 1] == c;
    let at_n2 = bs[n2 - 1] == c;
    (at_n1 && !at_n2) || (!at_n1 && at_n2)
  })
}

fn go(s: &str, f: fn(usize, usize, u8, &str) -> bool) -> usize {
  let r = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
  let mut ret = 0;
  for line in s.lines() {
    let caps = r.captures(line).unwrap();
    let n1: usize = caps[1].parse().unwrap();
    let n2: usize = caps[2].parse().unwrap();
    let c = caps[3].as_bytes()[0];
    let pw = &caps[4];
    if f(n1, n2, c, pw) {
      ret += 1;
    }
  }
  ret
}

#[test]
fn t() {
  let inp = include_str!("input/d02.txt");
  assert_eq!(p1(inp), 447);
  assert_eq!(p2(inp), 249);
}
