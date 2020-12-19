use crate::intcode::Intcode;
use helpers::permute::permute;

pub fn p1(s: &str) -> i64 {
  let p = Intcode::parse(s);
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

pub fn p2(s: &str) -> i64 {
  let p = Intcode::parse(s);
  let mut output = Vec::with_capacity(1);
  permute(5..=9)
    .into_iter()
    .map(|candidate| {
      let mut ps: Vec<_> = candidate
        .into_iter()
        .map(|phase| {
          let mut p = p.clone();
          p.input(phase);
          p
        })
        .collect();
      let mut inp = 0;
      loop {
        let mut iter = ps.iter_mut();
        let p = iter.next().unwrap();
        p.input(inp);
        let res = p.run(&mut output);
        inp = output.pop().unwrap();
        assert!(output.is_empty());
        for p in iter {
          p.input(inp);
          let other = p.run(&mut output);
          inp = output.pop().unwrap();
          assert!(output.is_empty());
          assert_eq!(res, other);
        }
        if res.is_done() {
          break inp;
        }
      }
    })
    .max()
    .unwrap()
}

#[test]
fn t() {
  let inp = include_str!("input/d07.txt");
  assert_eq!(p1(inp), 24625);
  assert_eq!(p2(inp), 36497698);
}
