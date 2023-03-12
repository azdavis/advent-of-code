use crate::intcode::{parse, Intcode, Res};
use std::cmp::Ordering;

pub fn p1(s: &str) -> usize {
  let mut p = Intcode::parse(s);
  let mut output = Vec::new();
  assert!(p.run(&mut output).is_done());
  assert_eq!(output.len() % 3, 0);
  // returns the wrong answer if output includes duplicate tiles
  output
    .chunks_exact(3)
    .filter(|ch| matches!(parse_tile(ch[2]), Tile::Block))
    .count()
}

pub fn p2(s: &str) -> i64 {
  let mut xs: Vec<_> = parse(s).collect();
  xs[0] = 2;
  let mut prog = Intcode::new(xs);
  let mut output = Vec::new();
  let mut score = 0;
  let mut paddle_x = 0;
  let mut ball_x = 0;
  loop {
    output.clear();
    let res = prog.run(&mut output);
    assert_eq!(output.len() % 3, 0);
    for ch in output.chunks_exact(3) {
      let x = ch[0];
      let y = ch[1];
      if x == -1 && y == 0 {
        score = ch[2];
        continue;
      }
      match parse_tile(ch[2]) {
        Tile::Empty | Tile::Wall | Tile::Block => {}
        Tile::HorizontalPaddle => paddle_x = x,
        Tile::Ball => ball_x = x,
      }
    }
    match res {
      Res::Done => return score,
      Res::NeedInput => {}
    }
    let inp = match paddle_x.cmp(&ball_x) {
      Ordering::Less => 1,
      Ordering::Equal => 0,
      Ordering::Greater => -1,
    };
    prog.input(inp);
  }
}

enum Tile {
  Empty,
  Wall,
  Block,
  HorizontalPaddle,
  Ball,
}

fn parse_tile(n: i64) -> Tile {
  match n {
    0 => Tile::Empty,
    1 => Tile::Wall,
    2 => Tile::Block,
    3 => Tile::HorizontalPaddle,
    4 => Tile::Ball,
    _ => panic!("bad tile: {n}"),
  }
}

#[test]
fn t() {
  let s = include_str!("input/d13.txt");
  assert_eq!(p1(s), 372);
  assert_eq!(p2(s), 19297);
}
