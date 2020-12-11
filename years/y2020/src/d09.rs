pub fn p1(s: &str) -> u64 {
  p1_help(&parse(s))
}

fn p1_help(nums: &[u64]) -> u64 {
  'outer: for (idx, &n) in nums.iter().enumerate().skip(WINDOW) {
    for &a in nums[idx - WINDOW..idx].iter() {
      for &b in nums[idx - WINDOW..idx].iter() {
        if a + b == n {
          continue 'outer;
        }
      }
    }
    return n;
  }
  panic!()
}

const WINDOW: usize = 25;

fn parse(s: &str) -> Vec<u64> {
  s.split('\n')
    .filter_map(|x| {
      if x.is_empty() {
        None
      } else {
        Some(x.parse().unwrap())
      }
    })
    .collect()
}
