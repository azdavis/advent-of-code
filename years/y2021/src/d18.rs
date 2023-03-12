use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Num {
  Val(u32),
  Pair(Box<Num>, Box<Num>),
}

impl Num {
  fn pair(a: Num, b: Num) -> Num {
    Num::Pair(Box::new(a), Box::new(b))
  }

  fn into_val(self) -> u32 {
    match self {
      Self::Val(v) => v,
      Self::Pair(..) => panic!("into_val not Val"),
    }
  }

  fn parse(s: &str) -> Num {
    let mut iter = s.chars().peekable();
    let ret = Num::parse_help(&mut iter);
    assert!(iter.next().is_none());
    ret
  }

  fn parse_help(it: &mut Peekable<Chars<'_>>) -> Num {
    let &c = it.peek().unwrap();
    if c == '[' {
      it.next().unwrap();
      let a = Num::parse_help(it);
      assert_eq!(it.next().unwrap(), ',');
      let b = Num::parse_help(it);
      assert_eq!(it.next().unwrap(), ']');
      return Num::pair(a, b);
    }
    let mut val = String::with_capacity(1);
    while let Some(&c) = it.peek() {
      if !c.is_ascii_digit() {
        break;
      }
      it.next().unwrap();
      val.push(c);
    }
    Num::Val(val.parse().unwrap())
  }
}

enum Ret {
  /// the bool is whether this exploded.
  Normal(Num, bool),
  ExplodeBoth(u32, u32),
  ExplodeLt(u32, Num),
  ExplodeRt(Num, u32),
}

fn explode(num: Num, depth: usize) -> Ret {
  match num {
    Num::Val(n) => Ret::Normal(Num::Val(n), false),
    Num::Pair(a, b) => {
      if depth == 4 {
        return Ret::ExplodeBoth(a.into_val(), b.into_val());
      }
      match explode(*a, depth + 1) {
        Ret::Normal(a, a_did) => {
          if a_did {
            Ret::Normal(Num::Pair(Box::new(a), b), true)
          } else {
            match explode(*b, depth + 1) {
              Ret::Normal(b, b_did) => Ret::Normal(Num::pair(a, b), b_did),
              Ret::ExplodeBoth(na, nb) => {
                Ret::ExplodeRt(Num::pair(add_to_rt(a, na), Num::Val(0)), nb)
              }
              Ret::ExplodeLt(na, b) => Ret::Normal(Num::pair(add_to_rt(a, na), b), true),
              Ret::ExplodeRt(b, nb) => Ret::ExplodeRt(Num::pair(a, b), nb),
            }
          }
        }
        Ret::ExplodeBoth(na, nb) => Ret::ExplodeLt(na, Num::pair(Num::Val(0), add_to_lt(*b, nb))),
        Ret::ExplodeLt(na, a) => Ret::ExplodeLt(na, Num::Pair(Box::new(a), b)),
        Ret::ExplodeRt(a, nb) => Ret::Normal(Num::pair(a, add_to_lt(*b, nb)), true),
      }
    }
  }
}

fn add_to_lt(num: Num, add: u32) -> Num {
  match num {
    Num::Val(n) => Num::Val(n + add),
    Num::Pair(a, b) => Num::Pair(Box::new(add_to_lt(*a, add)), b),
  }
}

fn add_to_rt(num: Num, add: u32) -> Num {
  match num {
    Num::Val(n) => Num::Val(n + add),
    Num::Pair(a, b) => Num::Pair(a, Box::new(add_to_rt(*b, add))),
  }
}

fn split(num: Num) -> (Num, bool) {
  match num {
    Num::Val(v) => {
      if v >= 10 {
        let a = v / 2;
        let n = Num::pair(Num::Val(a), Num::Val(v - a));
        (n, true)
      } else {
        (Num::Val(v), false)
      }
    }
    Num::Pair(a, b) => {
      let (a, did) = split(*a);
      if did {
        (Num::Pair(Box::new(a), b), true)
      } else {
        let (b, did) = split(*b);
        (Num::pair(a, b), did)
      }
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
enum ReduceKind {
  Explode,
  Split,
}

fn reduce_one(num: Num) -> (Num, Option<ReduceKind>) {
  match explode(num, 0) {
    Ret::Normal(n, did) => {
      if did {
        (n, Some(ReduceKind::Explode))
      } else {
        let (n, did) = split(n);
        (n, did.then_some(ReduceKind::Split))
      }
    }
    Ret::ExplodeBoth(_, _) => unreachable!(),
    Ret::ExplodeLt(_, num) | Ret::ExplodeRt(num, _) => (num, Some(ReduceKind::Explode)),
  }
}

fn reduce(mut num: Num) -> Num {
  loop {
    let (ret, kind) = reduce_one(num);
    if kind.is_some() {
      num = ret;
    } else {
      return ret;
    }
  }
}

fn add_and_reduce(a: Num, b: Num) -> Num {
  reduce(Num::pair(a, b))
}

fn magnitude(num: Num) -> u32 {
  match num {
    Num::Val(v) => v,
    Num::Pair(a, b) => (magnitude(*a) * 3) + (magnitude(*b) * 2),
  }
}

fn parse(s: &str) -> impl Iterator<Item = Num> + '_ {
  s.lines().map(Num::parse)
}

pub fn p1(s: &str) -> u32 {
  magnitude(parse(s).reduce(add_and_reduce).unwrap())
}

pub fn p2(s: &str) -> u32 {
  let ns: Vec<_> = parse(s).collect();
  ns.iter()
    .enumerate()
    .flat_map(|(i, n1)| {
      ns.iter().skip(i + 1).map(|n2| {
        magnitude(add_and_reduce(n1.clone(), n2.clone()))
          .max(magnitude(add_and_reduce(n2.clone(), n1.clone())))
      })
    })
    .max()
    .unwrap()
}

#[test]
fn t() {
  let s = include_str!("input/d18.txt");
  assert_eq!(p1(s), 3486);
  assert_eq!(p2(s), 4747);
}

#[test]
fn ex() {
  let a = Num::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
  let b = Num::parse("[1,1]");
  let ab = Num::pair(a, b);
  assert_eq!(Num::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"), ab);
  let (n, r) = reduce_one(ab);
  assert_eq!(r, Some(ReduceKind::Explode));
  assert_eq!(Num::parse("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"), n);
  let (n, r) = reduce_one(n);
  assert_eq!(r, Some(ReduceKind::Explode));
  assert_eq!(Num::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]"), n);
  let (n, r) = reduce_one(n);
  assert_eq!(r, Some(ReduceKind::Split));
  assert_eq!(Num::parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"), n);
  let (n, r) = reduce_one(n);
  assert_eq!(r, Some(ReduceKind::Split));
  assert_eq!(Num::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"), n);
  let (n, r) = reduce_one(n);
  assert_eq!(r, Some(ReduceKind::Explode));
  assert_eq!(Num::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), n);
  let (n, r) = reduce_one(n);
  assert!(r.is_none());
  assert_eq!(Num::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), n);
}
