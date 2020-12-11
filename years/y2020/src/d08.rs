use std::collections::HashSet;

pub fn p1(s: &str) -> i32 {
  let instrs = get_instrs(s);
  let mut visited = HashSet::new();
  let mut acc = 0;
  let mut idx = 0;
  loop {
    if !visited.insert(idx) {
      return acc;
    }
    let instr = instrs[idx];
    match instr.kind {
      InstrKind::Acc => {
        acc += instr.num;
        idx += 1;
      }
      InstrKind::Jmp => {
        // kind of awkward lol
        if instr.num < 0 {
          let neg_n = -instr.num as usize;
          assert!(idx >= neg_n);
          idx -= neg_n;
        } else {
          idx += instr.num as usize;
        }
      }
      InstrKind::Nop => idx += 1,
    }
  }
}

fn get_instrs(s: &str) -> Vec<Instr> {
  s.split('\n')
    .filter(|x| !x.is_empty())
    .map(Instr::parse)
    .collect()
}

#[derive(Debug, Clone, Copy)]
enum InstrKind {
  Acc,
  Jmp,
  Nop,
}

impl InstrKind {
  fn parse(s: &str) -> Self {
    match s {
      "acc" => Self::Acc,
      "jmp" => Self::Jmp,
      "nop" => Self::Nop,
      bad => panic!("invalid instr kind: {}", bad),
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Instr {
  kind: InstrKind,
  num: i32,
}

impl Instr {
  fn parse(s: &str) -> Self {
    let mut iter = s.split(' ');
    let kind = InstrKind::parse(iter.next().unwrap());
    let num: i32 = iter.next().unwrap().parse().unwrap();
    assert!(iter.next().is_none());
    Instr { kind, num }
  }
}
