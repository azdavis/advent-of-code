pub fn p1(s: &str) -> u64 {
  p1_help(&parse(s))
}

pub fn p2(s: &str) -> u64 {
  let nums = parse(s);
  let target = p1_help(&nums);
  for start in 0..(nums.len() - 1) {
    let mut acc = nums[start];
    for end in (start + 1)..(nums.len()) {
      acc += nums[end];
      if acc == target {
        let min = *nums[start..end].iter().min().unwrap();
        let max = *nums[start..end].iter().max().unwrap();
        return min + max;
      }
    }
  }
  panic!()
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
