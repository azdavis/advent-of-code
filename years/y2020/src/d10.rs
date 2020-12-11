pub fn p1(s: &str) -> usize {
  let mut nums = parse(s);
  // start at 0
  nums.push(0);
  nums.sort_unstable();
  let mut gap_1 = 0;
  // always 1 for the end
  let mut gap_3 = 1;
  for ns in nums.windows(2) {
    match ns[1] - ns[0] {
      1 => gap_1 += 1,
      2 => {}
      3 => gap_3 += 1,
      bad => panic!("bad gap: {}", bad),
    }
  }
  gap_1 * gap_3
}

fn parse(s: &str) -> Vec<u16> {
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
