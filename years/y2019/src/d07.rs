use crate::intcode::{parse, Intcode};
use helpers::permute::permute;

pub fn p1(s: &str) -> i32 {
  let p = Intcode::new(parse(s));
  let mut output = Vec::with_capacity(1);
  permute(0..=4)
    .into_iter()
    .map(|candidate| {
      candidate.into_iter().fold(0, |inp, phase| {
        let mut p = p.clone();
        p.input(phase);
        p.input(inp);
        assert!(p.run(&mut output).is_done());
        let out = output.pop().unwrap();
        assert!(output.is_empty());
        out
      })
    })
    .max()
    .unwrap()
}

pub fn p2(s: &str) -> i32 {
  todo!()
}

#[test]
fn t() {
  let inp = include_str!("input/d07.txt");
  assert_eq!(p1(inp), 24625);
  // assert_eq!(p2(inp), ___);
}
