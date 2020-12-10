pub fn p1(s: &str) -> usize {
  help(s, 3, 1)
}

pub fn p2(s: &str) -> usize {
  help(s, 1, 1) * help(s, 3, 1) * help(s, 5, 1) * help(s, 7, 1) * help(s, 1, 2)
}

fn help(s: &str, right: usize, down: usize) -> usize {
  let mut trees = 0;
  let mut y = 0;
  let line_len = s.split('\n').next().unwrap().len();
  let mut iter = s.split('\n');
  while let Some(line) = iter.next() {
    if line.is_empty() {
      continue;
    }
    let bs = line.as_bytes();
    if bs[y] == b'#' {
      trees += 1;
    }
    y += right;
    y %= line_len;
    for _ in 0..down - 1 {
      iter.next();
    }
  }
  trees
}
