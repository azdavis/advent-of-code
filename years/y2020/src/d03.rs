pub fn p1(s: &str) -> usize {
  let grid = parse(s);
  go(&grid, 3, 1)
}

pub fn p2(s: &str) -> usize {
  let grid = parse(s);
  go(&grid, 1, 1)
    * go(&grid, 3, 1)
    * go(&grid, 5, 1)
    * go(&grid, 7, 1)
    * go(&grid, 1, 2)
}

fn go(grid: &[&[u8]], right: usize, down: usize) -> usize {
  let mut ret = 0;
  let mut y = 0;
  let line_len = grid.first().unwrap().len();
  let mut iter = grid.iter();
  while let Some(row) = iter.next() {
    if row[y] == b'#' {
      ret += 1;
    }
    y += right;
    y %= line_len;
    for _ in 0..down - 1 {
      iter.next();
    }
  }
  ret
}

fn parse(s: &str) -> Vec<&[u8]> {
  s.split('\n')
    .filter(|line| !line.is_empty())
    .map(|line| line.as_bytes())
    .collect()
}
