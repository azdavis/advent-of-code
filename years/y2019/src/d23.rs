use crate::intcode::Intcode;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Part {
  P1,
  P2,
}

fn run(s: &str, part: Part) -> i64 {
  let prog = Intcode::parse(s);
  let mut nodes = vec![(prog, VecDeque::<(i64, i64)>::new()); 50];
  for (addr, (prog, _)) in nodes.iter_mut().enumerate() {
    prog.input(addr.try_into().unwrap());
  }
  let mut output = Vec::<i64>::new();
  let mut nat_cur = (0i64, 0i64);
  let mut nat_prev_y = 0i64;
  loop {
    for idx in 0..nodes.len() {
      let (prog, queue) = &mut nodes[idx];
      if queue.is_empty() {
        prog.input(-1);
      } else {
        for (x, y) in queue.drain(..) {
          prog.input(x);
          prog.input(y);
        }
      }
      assert!(output.is_empty());
      assert!(prog.run(&mut output).needs_input());
      assert_eq!(output.len() % 3, 0);
      for chunk in output.chunks_exact(3) {
        let addr: usize = chunk[0].try_into().unwrap();
        let x = chunk[1];
        let y = chunk[2];
        if addr == 255 {
          match part {
            Part::P1 => return y,
            Part::P2 => nat_cur = (x, y),
          }
        } else {
          nodes[addr].1.push_back((x, y));
        }
      }
      output.clear();
    }
    match part {
      Part::P1 => continue,
      Part::P2 => {}
    }
    if nodes.iter().all(|(_, queue)| queue.is_empty()) {
      nodes[0].1.push_back(nat_cur);
      if nat_prev_y == nat_cur.1 {
        return nat_prev_y;
      }
      nat_prev_y = nat_cur.1;
    }
  }
}

pub fn p1(s: &str) -> i64 {
  run(s, Part::P1)
}

pub fn p2(s: &str) -> i64 {
  run(s, Part::P2)
}

#[test]
fn t() {
  let s = include_str!("input/d23.txt");
  assert_eq!(p1(s), 27182);
  assert_eq!(p2(s), 19285);
}
