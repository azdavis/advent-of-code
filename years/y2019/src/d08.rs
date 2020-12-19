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

pub fn p2(s: &str) -> u32 {
  todo!()
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn parse(s: &str) -> Vec<Pixel> {
  s.split('\n')
    .next()
    .unwrap()
    .chars()
    .map(Pixel::parse)
    .collect()
}

#[derive(Debug, Clone, Copy)]
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
  // assert_eq!(p2(inp), ___);
}
