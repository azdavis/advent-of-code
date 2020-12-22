pub fn p1(s: &str) -> u64 {
  go(&parse(s))
}

pub fn p2(s: &str) -> u64 {
  let nums = parse(s);
  let target = go(&nums);
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
  panic!("no solution")
}

fn go(nums: &[u64]) -> u64 {
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
  panic!("no solution")
}

const WINDOW: usize = 25;

fn parse(s: &str) -> Vec<u64> {
  s.lines().map(|x| x.parse().unwrap()).collect()
}

#[test]
fn t() {
  let inp = include_str!("input/d09.txt");
  assert_eq!(p1(inp), 675280050);
  assert_eq!(p2(inp), 96081673);
}
