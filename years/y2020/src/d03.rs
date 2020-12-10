pub fn p1(s: &str) {
  let mut trees = 0;
  let mut y = 0;
  let line_len = s.split('\n').next().unwrap().len();
  for line in s.split('\n') {
    if line.is_empty() {
      continue;
    }
    let bs = line.as_bytes();
    if bs[y] == b'#' {
      trees += 1;
    }
    y += 3;
    y %= line_len;
  }
  println!("{}", trees);
}
