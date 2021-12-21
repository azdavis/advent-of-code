use helpers::HashSet;

pub fn p1(s: &str) -> usize {
  s.lines()
    .filter(|&line| {
      let mut set = HashSet::<&str>::default();
      line.split(' ').all(|word| set.insert(word))
    })
    .count()
}

pub fn p2(s: &str) -> usize {
  s.lines()
    .filter(|&line| {
      let mut set = HashSet::<Vec<u8>>::default();
      line.split(' ').all(|word| {
        let mut cs = word.to_owned().into_bytes();
        cs.sort_unstable();
        set.insert(cs)
      })
    })
    .count()
}

#[test]
fn t() {
  let s = include_str!("input/d04.txt");
  assert_eq!(p1(s), 325);
  assert_eq!(p2(s), 119);
}
