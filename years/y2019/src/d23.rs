use crate::intcode::Intcode;
use std::collections::VecDeque;

pub fn p1(s: &str) -> i64 {
  let prog = Intcode::parse(s);
  let mut nodes = vec![(prog, VecDeque::<(i64, i64)>::new()); 50];
  for (addr, (prog, _)) in nodes.iter_mut().enumerate() {
    prog.input(addr.try_into().unwrap());
  }
  let mut output = Vec::<i64>::new();
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
          return y;
        }
        nodes[addr].1.push_front((x, y));
      }
      output.clear();
    }
  }
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  let s = include_str!("input/d23.txt");
  assert_eq!(p1(s), 27182);
  // assert_eq!(p2(s), ___);
}
