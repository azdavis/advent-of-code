use regex::Regex;

pub fn p1(s: &str) {
  help(s, |lo, hi, want, pw| {
    let count = pw.as_bytes().iter().filter(|&&b| b == want).count();
    lo <= count && count <= hi
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
    let lo: usize = caps[1].parse().unwrap();
    let hi: usize = caps[2].parse().unwrap();
    let want = caps[3].as_bytes()[0];
    let pw = &caps[4];
    if f(lo, hi, want, pw) {
      ok += 1;
    }
  }
  println!("{}", ok);
}
