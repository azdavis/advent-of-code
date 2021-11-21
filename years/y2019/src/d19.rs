use crate::intcode::Intcode;

pub fn p1(s: &str) -> usize {
  let prog = Intcode::parse(s);
  let mut output = Vec::with_capacity(1);
  (0i64..50)
    .map(|x| {
      (0i64..50)
        .filter(|&y| {
          let mut prog = prog.clone();
          prog.input(x);
          prog.input(y);
          assert!(output.is_empty());
          assert!(prog.run(&mut output).is_done());
          match output.pop().unwrap() {
            0 => false,
            1 => true,
            out => panic!("bad out: {}", out),
          }
        })
        .count()
    })
    .sum()
}

pub fn p2(_: &str) -> u32 {
  todo!()
}

#[test]
fn t() {
  let s = include_str!("input/d19.txt");
  assert_eq!(p1(s), 197);
  // assert_eq!(p2(s), ___);
}
