const RESTART: usize = 6;
const MAX: usize = RESTART + 2;

fn run(s: &str, rounds: usize) -> usize {
  let mut fish = vec![0usize; MAX + 1];
  for idx in s.trim().split(',') {
    let idx: usize = idx.parse().unwrap();
    fish[idx] += 1;
  }
  for _ in 0..rounds {
    fish = (0usize..=MAX)
      .map(|idx| {
        if idx == RESTART {
          fish[0] + fish[RESTART + 1]
        } else if idx == MAX {
          fish[0]
        } else {
          fish[idx + 1]
        }
      })
      .collect();
  }
  fish.into_iter().sum()
}

pub fn p1(s: &str) -> usize {
  run(s, 80)
}

pub fn p2(s: &str) -> usize {
  run(s, 256)
}

#[test]
fn t() {
  let s = include_str!("input/d06.txt");
  assert_eq!(p1(s), 371_379);
  assert_eq!(p2(s), 1_674_303_997_472);
}
