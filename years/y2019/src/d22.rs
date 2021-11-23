#[derive(Debug, Clone, Copy)]
enum Instr {
  NewStack,
  Cut(i16),
  Incr(usize),
}

fn parse(s: &str) -> impl Iterator<Item = Instr> + '_ {
  s.lines().map(|line| {
    let mut iter = line.split_ascii_whitespace();
    match iter.next().unwrap() {
      "cut" => {
        let n: i16 = iter.next().unwrap().parse().unwrap();
        assert!(iter.next().is_none());
        Instr::Cut(n)
      }
      "deal" => match iter.next().unwrap() {
        "with" => {
          assert_eq!(iter.next(), Some("increment"));
          let n: usize = iter.next().unwrap().parse().unwrap();
          assert!(iter.next().is_none());
          Instr::Incr(n)
        }
        "into" => {
          assert_eq!(iter.next(), Some("new"));
          assert_eq!(iter.next(), Some("stack"));
          assert!(iter.next().is_none());
          Instr::NewStack
        }
        deal => panic!("unknown deal: {}", deal),
      },
      instr => panic!("unknown instr: {}", instr),
    }
  })
}

fn run(s: &str, len: u16) -> Vec<u16> {
  let mut deck: Vec<_> = (0u16..len).collect();
  let len = len as usize;
  for instr in parse(s) {
    match instr {
      Instr::NewStack => deck.reverse(),
      Instr::Cut(cut) => {
        let cut = if cut >= 0 { cut } else { (len as i16) + cut };
        let mut bot = deck.split_off(cut as usize);
        bot.extend(deck);
        deck = bot;
      }
      Instr::Incr(inc) => {
        let mut new_deck = vec![0u16; len];
        let mut idx = 0usize;
        for card in deck {
          new_deck[idx] = card;
          idx += inc;
          idx %= len;
        }
        deck = new_deck;
      }
    }
  }
  deck
}

pub fn p1(s: &str) -> usize {
  let deck = run(s, 10007);
  deck.iter().position(|&c| c == 2019).unwrap()
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  let s = include_str!("input/d22.txt");
  assert_eq!(p1(s), 4703);
  // assert_eq!(p2(s), ___);
}

#[cfg(test)]
mod examples {
  use super::run;

  #[test]
  fn inc_3_explanation() {
    assert_eq!(
      run("deal with increment 3", 10),
      [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
    );
  }

  #[test]
  fn t1() {
    assert_eq!(
      run(include_str!("input/d22_ex1.txt"), 10),
      [0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
    );
  }

  #[test]
  fn t2() {
    assert_eq!(
      run(include_str!("input/d22_ex2.txt"), 10),
      [3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
    );
  }

  #[test]
  fn t3() {
    assert_eq!(
      run(include_str!("input/d22_ex3.txt"), 10),
      [6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
    );
  }

  #[test]
  fn t4() {
    assert_eq!(
      run(include_str!("input/d22_ex4.txt"), 10),
      [9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
    );
  }
}
