use helpers::maplit::hashmap;

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

pub fn p2(s: &str) -> usize {
  let mut nums = parse(s);
  nums.sort_unstable();
  let mut dp = hashmap![0 => 1];
  for &n in nums.iter() {
    let ans: usize = [1, 2, 3]
      .iter()
      .filter_map(|&gap| dp.get(&n.checked_sub(gap)?))
      .sum();
    dp.insert(n, ans);
  }
  *dp.get(nums.last().unwrap()).unwrap()
}

fn parse(s: &str) -> Vec<u16> {
  s.lines().map(|x| x.parse().unwrap()).collect()
}

#[test]
fn t() {
  let inp = include_str!("input/d10.txt");
  assert_eq!(p1(inp), 1700);
  assert_eq!(p2(inp), 12401793332096);
}
