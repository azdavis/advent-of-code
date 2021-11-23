use helpers::block_char::{EMPTY, FILLED};

pub fn p1(s: &str) -> usize {
  let ns = parse(s);
  let least_black = ns
    .chunks(WIDTH * HEIGHT)
    .min_by_key(|xs| xs.iter().filter(|&x| matches!(*x, Pixel::Black)).count())
    .unwrap();
  let num_white = least_black
    .iter()
    .filter(|&x| matches!(*x, Pixel::White))
    .count();
  let num_transparent = least_black
    .iter()
    .filter(|&x| matches!(*x, Pixel::Transparent))
    .count();
  num_white * num_transparent
}

pub fn p2(s: &str) -> String {
  let ns = parse(s);
  let mut ret = String::new();
  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      let mut layer = 0;
      let px = loop {
        match ns.get((layer * WIDTH * HEIGHT) + (y * WIDTH) + x) {
          None => break Pixel::Transparent,
          Some(&Pixel::Transparent) => layer += 1,
          Some(&x) => break x,
        }
      };
      let c = match px {
        Pixel::Black => FILLED,
        Pixel::White => EMPTY,
        Pixel::Transparent => ' ',
      };
      ret.push(c);
    }
    ret.push('\n')
  }
  ret
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn parse(s: &str) -> Vec<Pixel> {
  s.trim_end().chars().map(Pixel::parse).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
  Black,
  White,
  Transparent,
}

impl Pixel {
  fn parse(c: char) -> Self {
    match c {
      '0' => Self::Black,
      '1' => Self::White,
      '2' => Self::Transparent,
      _ => panic!("bad pixel: {}", c),
    }
  }
}

#[test]
fn t() {
  let s = include_str!("input/d08.txt");
  assert_eq!(p1(s), 2760);
  let out = include_str!("snapshots/d08p2.txt");
  assert_eq!(p2(s), out);
}
