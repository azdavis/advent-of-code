use regex::Regex;

pub fn p1(s: &str) {
  help(s, |n1, n2, c, pw| {
    let count = pw.as_bytes().iter().filter(|&&b| b == c).count();
    n1 <= count && count <= n2
  })
}

pub fn p2(s: &str) {
  help(s, |n1, n2, c, pw| {
    let bs = pw.as_bytes();
    let at_n1 = bs[n1 - 1] == c;
    let at_n2 = bs[n2 - 1] == c;
    (at_n1 && !at_n2) || (!at_n1 && at_n2)
  })
}

fn help<F>(s: &str, f: F)
where
  F: Fn(usize, usize, u8, &str) -> bool,
{
  let r = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
  let mut ok = 0;
  for line in s.split('\n') {
    if line.is_empty() {
      continue;
    }
    let caps = r.captures(line).unwrap();
    let n1: usize = caps[1].parse().unwrap();
    let n2: usize = caps[2].parse().unwrap();
    let c = caps[3].as_bytes()[0];
    let pw = &caps[4];
    if f(n1, n2, c, pw) {
      ok += 1;
    }
  }
  println!("{}", ok);
}
