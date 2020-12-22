use std::fmt;

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

#[allow(clippy::needless_range_loop)]
pub fn p2(s: &str) -> Img {
  let ns = parse(s);
  let mut pixels = vec![vec![Pixel::Transparent; WIDTH]; HEIGHT];
  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      let mut layer = 0;
      pixels[y][x] = loop {
        match ns.get((layer * WIDTH * HEIGHT) + (y * WIDTH) + x) {
          None => break Pixel::Transparent,
          Some(&Pixel::Transparent) => layer += 1,
          Some(&x) => break x,
        }
      };
    }
  }
  Img { pixels }
}

#[derive(Debug)]
pub struct Img {
  pixels: Vec<Vec<Pixel>>,
}

impl fmt::Display for Img {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut rows = self.pixels.iter();
    if let Some(row) = rows.next() {
      fmt_row(row, f)?;
    }
    for row in rows {
      writeln!(f)?;
      fmt_row(row, f)?;
    }
    Ok(())
  }
}

fn fmt_row(row: &[Pixel], f: &mut fmt::Formatter<'_>) -> fmt::Result {
  for px in row.iter() {
    match *px {
      Pixel::Black => write!(f, "█")?,
      Pixel::White => write!(f, "░")?,
      Pixel::Transparent => write!(f, " ")?,
    }
  }
  Ok(())
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
  let inp = include_str!("input/d08.txt");
  assert_eq!(p1(inp), 2760);
  use Pixel::{Black as B, White as W};
  #[rustfmt::skip]
  let p2_out = [
    [B, W, W, B, B, B, W, W, B, B, W, B, B, W, B, W, W, W, W, B, W, W, W, B, B],
    [W, B, B, W, B, W, B, B, W, B, W, B, B, W, B, W, B, B, B, B, W, B, B, W, B],
    [W, B, B, W, B, W, B, B, B, B, W, B, B, W, B, W, W, W, B, B, W, W, W, B, B],
    [W, W, W, W, B, W, B, W, W, B, W, B, B, W, B, W, B, B, B, B, W, B, B, W, B],
    [W, B, B, W, B, W, B, B, W, B, W, B, B, W, B, W, B, B, B, B, W, B, B, W, B],
    [W, B, B, W, B, B, W, W, W, B, B, W, W, B, B, W, W, W, W, B, W, W, W, B, B],
  ];
  assert_eq!(p2(inp).pixels, p2_out);
}
