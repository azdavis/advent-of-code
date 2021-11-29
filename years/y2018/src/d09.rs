use helpers::{Lazy, Regex};

static RE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r#"^(\d+) players; last marble is worth (\d+) points$"#).unwrap()
});

fn parse(s: &str) -> (usize, u32) {
  let caps = RE.captures(s.trim()).unwrap();
  (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

fn run(n_players: usize, last: u32) -> u32 {
  let mut marbles = vec![0u32];
  let mut scores = vec![0u32; n_players];
  let mut next_marble = 1u32;
  let mut cur_idx = 0usize;
  let mut cur_player = 0usize;
  loop {
    if next_marble % 23 == 0 {
      while cur_idx < 7 {
        cur_idx += marbles.len();
      }
      cur_idx -= 7;
      let rm = marbles.remove(cur_idx);
      scores[cur_player] += rm + next_marble;
    } else {
      cur_idx += 2;
      cur_idx %= marbles.len();
      marbles.insert(cur_idx, next_marble);
    }
    if next_marble == last {
      return scores.iter().copied().max().unwrap();
    }
    next_marble += 1;
    cur_player += 1;
    cur_player %= n_players;
  }
}

pub fn p1(s: &str) -> u32 {
  let (n_players, last) = parse(s);
  run(n_players, last)
}

pub fn p2(s: &str) -> u32 {
  let (n_players, last) = parse(s);
  run(n_players, last * 100)
}

#[test]
fn t() {
  let s = include_str!("input/d09.txt");
  assert_eq!(p1(s), 399645);
  // assert_eq!(p2(s), 0);
}

#[cfg(test)]
#[rustfmt::skip]
mod examples {
  use super::run;
  #[test] fn ex1() { assert_eq!(run(9, 25), 32); }
  #[test] fn ex2() { assert_eq!(run(10, 1618), 8317); }
  #[test] fn ex3() { assert_eq!(run(13, 7999), 146373); }
  #[test] fn ex4() { assert_eq!(run(17, 1104), 2764); }
  #[test] fn ex5() { assert_eq!(run(21, 6111), 54718); }
  #[test] fn ex6() { assert_eq!(run(30, 5807), 37305); }
}
