use crate::intcode::Intcode;
use helpers::compass::Compass;
use helpers::vec2::Vec2;
use std::collections::{HashSet, VecDeque};

pub fn p1(s: &str) -> usize {
  match go(Intcode::parse(s)) {
    Res::NotFound(_) => panic!("couldn't find oxygen"),
    Res::Found(n, _) => n,
  }
}

pub fn p2(s: &str) -> usize {
  let prog = match go(Intcode::parse(s)) {
    Res::NotFound(_) => panic!("couldn't find oxygen"),
    Res::Found(_, x) => x,
  };
  match go(prog) {
    Res::NotFound(n) => n,
    Res::Found(..) => panic!("found oxygen twice"),
  }
}

enum Res {
  NotFound(usize),
  Found(usize, Intcode),
}

/// breadth-first search
fn go(prog: Intcode) -> Res {
  let mut visited = HashSet::new();
  let mut queue = VecDeque::from(vec![(Vec2::default(), prog)]);
  let mut level = 1;
  let mut output = Vec::with_capacity(1);
  loop {
    for _ in 0..queue.len() {
      let (point, prog) = queue.pop_front().unwrap();
      visited.insert(point);
      for &(compass, neighbor) in point.neighbors().iter() {
        // note: having the visited check be here means replacing the queue with
        // a stack does not yield a DFS. we put the check here since we don't
        // want to return Found when finding an already visited spot. also cuts
        // down on cloning.
        if visited.contains(&neighbor) {
          continue;
        }
        let mut p = prog.clone();
        p.input(compass_to_input(compass));
        assert!(p.run(&mut output).needs_input());
        match output.pop().unwrap() {
          0 => {}
          1 => queue.push_back((neighbor, p)),
          2 => return Res::Found(level, p),
          bad => panic!("bad output: {}", bad),
        }
        assert!(output.is_empty());
      }
    }
    if queue.is_empty() {
      return Res::NotFound(level - 1);
    }
    level += 1;
  }
}

fn compass_to_input(c: Compass) -> i64 {
  match c {
    Compass::North => 1,
    Compass::South => 2,
    Compass::West => 3,
    Compass::East => 4,
  }
}

#[test]
fn t() {
  let s = include_str!("input/d15.txt");
  assert_eq!(p1(s), 272);
  assert_eq!(p2(s), 398);
}