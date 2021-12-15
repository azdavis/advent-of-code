use helpers::HashMap;

fn run(s: &str, rounds: usize) -> usize {
  let mut lines = s.lines();
  let mut polymer = lines.next().unwrap().to_owned().into_bytes();
  assert!(lines.next().unwrap().is_empty());
  let rules: HashMap<_, _> = lines
    .map(|line| {
      let (a, b) = line.split_once(" -> ").unwrap();
      (a.as_bytes(), b.bytes().next().unwrap())
    })
    .collect();
  for _ in 0..rounds {
    let mut new_polymer = Vec::with_capacity(polymer.len() * 2);
    for w in polymer.windows(2) {
      new_polymer.push(w[0]);
      let &x = rules.get(w).unwrap();
      new_polymer.push(x);
    }
    new_polymer.push(polymer.pop().unwrap());
    polymer = new_polymer;
  }
  let mut counts = HashMap::<u8, usize>::default();
  for b in polymer {
    *counts.entry(b).or_default() += 1;
  }
  let min = counts.values().copied().min().unwrap();
  let max = counts.values().copied().max().unwrap();
  max - min
}

pub fn p1(s: &str) -> usize {
  run(s, 10)
}

pub fn p2(s: &str) -> usize {
  run(s, 40)
}

#[test]
fn t() {
  let s = include_str!("input/d14.txt");
  assert_eq!(p1(s), 3831);
  // assert_eq!(p2(s), 0);
}
