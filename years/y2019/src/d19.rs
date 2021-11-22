use crate::intcode::Intcode;
use helpers::HashMap;

struct Beam {
  prog: Intcode,
  out: Vec<i64>,
}

impl Beam {
  fn new(s: &str) -> Self {
    Self {
      prog: Intcode::parse(s),
      out: Vec::with_capacity(1),
    }
  }

  fn affects(&mut self, x: i64, y: i64) -> bool {
    assert!(self.out.is_empty());
    let mut prog = self.prog.clone();
    prog.input(x);
    prog.input(y);
    assert!(prog.run(&mut self.out).is_done());
    match self.out.pop().unwrap() {
      0 => false,
      1 => true,
      out => panic!("bad out: {}", out),
    }
  }
}

pub fn p1(s: &str) -> usize {
  let mut beam = Beam::new(s);
  (0i64..50)
    .map(|x| (0i64..50).filter(|&y| beam.affects(x, y)).count())
    .sum()
}

pub fn hm(s: &str) -> i32 {
  let wx = 928;
  let wy = 1033;
  let mut beam = Beam::new(s);
  for y in 1000i64..1300 {
    for x in 900i64..1050 {
      let in_sq = (wx..wx + 100).contains(&x) && (wy..wy + 100).contains(&y);
      let c = match ((x, y) == (wx, wy), beam.affects(x, y), in_sq) {
        (true, _, _) => '$',
        (_, false, false) => '.',
        (_, false, true) => '!',
        (_, true, false) => '#',
        (_, true, true) => 'O',
      }; /* if (x, y) == (928, 1033) {
           'O'
         } else if  {
           '#'
         } else {
           '.'
         }; */
      print!("{}", c);
    }
    println!();
  }
  0
}

pub fn p2(s: &str) -> u64 {
  let mut beam = Beam::new(s);
  let mut top_x = 100i64;
  let mut top_y = 0;
  while !beam.affects(top_x, top_y) {
    // eprintln!("aa");
    top_y += 1;
  }
  let mut bot_xs = HashMap::<i64, i64>::default();
  let mut bot_x = 0i64;
  while !beam.affects(bot_x, top_y - 1) {
    // eprintln!("bb");
    bot_x += 1
  }
  bot_xs.insert(top_y - 1, bot_x);
  // assert!(beam.affects(top_x, top_y));
  loop {
    while !beam.affects(top_x, top_y) {
      top_x += 1;
    }
    // eprintln!("b");
    while beam.affects(top_x + 1, top_y) {
      top_x += 1;
    }
    eprintln!("top {} {}", top_x, top_y);
    let mut bot_y = top_y;
    loop {
      // eprintln!("c");
      let bot_x = match bot_xs.get(&bot_y) {
        Some(x) => *x,
        None => {
          let mut x = bot_xs[&(bot_y - 1)];
          while !beam.affects(x, bot_y) {
            // eprintln!("d {} {}", x, bot_y);
            x += 1;
          }
          bot_xs.insert(bot_y, x);
          x
        }
      };
      // eprintln!("bot {} {}", bot_x, bot_y);
      if bot_x > top_x {
        break;
      }
      if top_x - bot_x >= 99 && bot_y - top_y >= 99 {
        eprintln!("{} {}", bot_x, top_y);
        return u64::try_from(bot_x * 10000 + top_y).unwrap();
      }
      bot_y += 1;
    }
    top_y += 1;
  }
}

#[test]
fn t() {
  let s = include_str!("input/d19.txt");
  assert_eq!(p1(s), 197);
  assert_eq!(p2(s), 9181022);
}
