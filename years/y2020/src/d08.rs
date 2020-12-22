use std::collections::HashSet;

pub fn p1(s: &str) -> i32 {
  let instrs = get_instrs(s);
  let res = simulate(&instrs);
  assert!(matches!(res.kind, ResKind::Loop));
  res.acc
}

pub fn p2(s: &str) -> i32 {
  let mut instrs = get_instrs(s);
  for idx in 0..instrs.len() {
    instrs[idx].kind = match instrs[idx].kind {
      InstrKind::Acc => continue,
      InstrKind::Jmp => InstrKind::Nop,
      InstrKind::Nop => InstrKind::Jmp,
    };
    let res = simulate(&instrs);
    match res.kind {
      ResKind::Terminate => return res.acc,
      ResKind::Loop => {}
    }
    instrs[idx].kind = match instrs[idx].kind {
      InstrKind::Acc => InstrKind::Acc,
      InstrKind::Jmp => InstrKind::Nop,
      InstrKind::Nop => InstrKind::Jmp,
    };
  }
  panic!("no solution")
}

fn simulate(instrs: &[Instr]) -> Res {
  let mut visited = HashSet::new();
  let mut acc = 0;
  let mut idx = 0;
  loop {
    if !visited.insert(idx) {
      return Res {
        acc,
        kind: ResKind::Loop,
      };
    }
    let instr = match instrs.get(idx) {
      Some(&x) => x,
      None => {
        return Res {
          acc,
          kind: ResKind::Terminate,
        }
      }
    };
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

struct Res {
  acc: i32,
  kind: ResKind,
}

enum ResKind {
  Loop,
  Terminate,
}

fn get_instrs(s: &str) -> Vec<Instr> {
  s.lines().map(Instr::parse).collect()
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
      _ => panic!("bad instr kind: {}", s),
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
    Self { kind, num }
  }
}

#[test]
fn t() {
  let inp = include_str!("input/d08.txt");
  assert_eq!(p1(inp), 1709);
  assert_eq!(p2(inp), 1976);
}
