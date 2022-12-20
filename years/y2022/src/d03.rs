use helpers::HashSet;

pub fn p1(s: &str) -> u32 {
  s.lines()
    .map(|line| {
      let line = line.as_bytes();
      assert_eq!(line.len() % 2, 0);
      let half = line.len() / 2;
      let s1 = byte_set(&line[..half]);
      let s2 = byte_set(&line[half..]);
      let &same = s1.intersection(&s2).next().unwrap();
      priority(same)
    })
    .sum()
}

pub fn p2(s: &str) -> u32 {
  let lines: Vec<_> = s.lines().collect();
  lines
    .chunks_exact(3)
    .map(|cs| {
      let s = cs
        .iter()
        .map(|x| byte_set(x.as_bytes()))
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .unwrap();
      assert_eq!(s.len(), 1);
      let same = s.into_iter().next().unwrap();
      priority(same)
    })
    .sum()
}

fn byte_set(xs: &[u8]) -> HashSet<u8> {
  xs.iter().copied().collect()
}

fn priority(n: u8) -> u32 {
  let ret = if n <= b'Z' {
    n - b'A' + 27
  } else {
    n - b'a' + 1
  };
  u32::from(ret)
}

#[test]
fn t() {
  let s = include_str!("input/d03.txt");
  assert_eq!(p1(s), 7990);
  assert_eq!(p2(s), 2602);
}
