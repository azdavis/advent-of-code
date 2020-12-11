use std::collections::HashSet;

pub fn p1(s: &str) -> i32 {
  let instrs: Vec<_> = s
    .split('\n')
    .filter(|x| !x.is_empty())
    .map(Instr::parse)
    .collect();
  let mut visited = HashSet::new();
  let mut acc = 0;
  let mut idx = 0;
  loop {
    if !visited.insert(idx) {
      return acc;
    }
    match instrs[idx] {
      Instr::Acc(n) => {
        acc += n;
        idx += 1;
      }
      Instr::Jmp(n) => {
        // kind of awkward lol
        if n < 0 {
          let neg_n = -n as usize;
          assert!(idx >= neg_n);
          idx -= neg_n;
        } else {
          idx += n as usize;
        }
      }
      Instr::Nop => idx += 1,
    }
  }
}

enum Instr {
  Acc(i32),
  Jmp(i32),
  Nop,
}

impl Instr {
  fn parse(s: &str) -> Self {
    let mut iter = s.split(' ');
    let kind = iter.next().unwrap();
    let num = iter.next().unwrap();
    assert!(iter.next().is_none());
    match kind {
      "acc" => Self::Acc(num.parse().unwrap()),
      "jmp" => Self::Jmp(num.parse().unwrap()),
      "nop" => Self::Nop,
      bad => panic!("invalid instr: {}", bad),
    }
  }
}
