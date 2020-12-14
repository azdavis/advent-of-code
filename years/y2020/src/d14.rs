use std::collections::HashMap;

pub fn p1(s: &str) -> usize {
  go(s, |mask, mem, addr, mut val| {
    val |= mask.on;
    val &= mask.off;
    mem.insert(addr, val);
  })
}

pub fn p2(_: &str) -> usize {
  todo!()
}

type Mem = HashMap<usize, usize>;

fn go<F>(s: &str, f: F) -> usize
where
  F: Fn(&Mask, &mut Mem, usize, usize),
{
  let instrs = parse(s);
  let mut mask = Mask::default();
  let mut mem = Mem::new();
  for instr in instrs {
    match instr {
      Instr::Mask(m) => mask = m,
      Instr::Mem(addr, val) => f(&mask, &mut mem, addr, val),
    }
  }
  mem.values().copied().sum()
}

fn parse(s: &str) -> Vec<Instr> {
  s.split('\n')
    .filter(|line| !line.is_empty())
    .map(Instr::parse)
    .collect()
}

#[derive(Debug, Default, Clone)]
struct Mask {
  on: usize,
  off: usize,
}

#[derive(Debug)]
enum Instr {
  Mask(Mask),
  Mem(usize, usize),
}

impl Instr {
  fn parse(s: &str) -> Self {
    let mut parts = s.split(' ');
    let fst = parts.next().unwrap();
    if fst == "mask" {
      assert_eq!(parts.next().unwrap(), "=");
      let mask = parts.next().unwrap();
      assert!(parts.next().is_none());
      let mut on = 0;
      let mut off = 0;
      for (idx, c) in mask.chars().rev().enumerate() {
        match c {
          'X' => {}
          '0' => off |= 1 << idx,
          '1' => on |= 1 << idx,
          _ => panic!("bad mask char: {}", c),
        }
      }
      off = !off;
      return Self::Mask(Mask { on, off });
    }
    let mut fst_parts = fst.split('[');
    assert_eq!(fst_parts.next().unwrap(), "mem");
    let mut mem_parts = fst_parts.next().unwrap().split(']');
    let addr: usize = mem_parts.next().unwrap().parse().unwrap();
    assert_eq!(mem_parts.next().unwrap(), "");
    assert!(mem_parts.next().is_none());
    assert_eq!(parts.next().unwrap(), "=");
    let val: usize = parts.next().unwrap().parse().unwrap();
    assert!(parts.next().is_none());
    Self::Mem(addr, val)
  }
}
