mod cycle_zipper;

use cycle_zipper::CycleZipper;
use helpers::static_regex;

static_regex!(RE = r#"^(\d+) players; last marble is worth (\d+) points$"#);

fn parse(s: &str) -> (usize, u32) {
  let caps = RE.captures(s.trim()).unwrap();
  (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

fn run(n_players: usize, last: u32) -> u32 {
  let mut marbles = CycleZipper::<u32>::new(0);
  let mut scores = vec![0u32; n_players];
  let mut player = 0usize;
  for marble in 1u32..=last {
    if marble % 23 == 0 {
      for _ in 0..7 {
        marbles.move_prev();
      }
      scores[player] += marbles.pop() + marble;
    } else {
      for _ in 0..2 {
        marbles.move_next();
      }
      marbles.push(marble);
    }
    player += 1;
    player %= n_players;
  }
  scores.iter().copied().max().unwrap()
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
  assert_eq!(p1(s), 399_645);
  assert_eq!(p2(s), 3_352_507_536);
}

#[cfg(test)]
mod examples {
  use super::run;

  #[test]
  fn ex1() {
    assert_eq!(run(9, 25), 32);
  }

  #[test]
  fn ex2() {
    assert_eq!(run(10, 1618), 8317);
  }

  #[test]
  fn ex3() {
    assert_eq!(run(13, 7999), 146_373);
  }

  #[test]
  fn ex4() {
    assert_eq!(run(17, 1104), 2764);
  }

  #[test]
  fn ex5() {
    assert_eq!(run(21, 6111), 54718);
  }

  #[test]
  fn ex6() {
    assert_eq!(run(30, 5807), 37305);
  }
}
