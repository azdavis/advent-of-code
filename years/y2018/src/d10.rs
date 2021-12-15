use helpers::{block_char, static_regex, HashSet};
use std::io::{self, Write};

static_regex!(
  RE = r#"^position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>$"#
);

type Vec2 = [isize; 2];

fn parse(s: &str) -> (Vec<Vec2>, Vec<Vec2>) {
  s.lines()
    .map(|line| {
      let caps = RE.captures(line).unwrap();
      let position: Vec2 = [caps[1].parse().unwrap(), caps[2].parse().unwrap()];
      let velocity: Vec2 = [caps[3].parse().unwrap(), caps[4].parse().unwrap()];
      (position, velocity)
    })
    .unzip()
}

fn get_bounding_corners(position: &[Vec2]) -> [Vec2; 2] {
  let min_x = position.iter().map(|&[x, _]| x).min().unwrap();
  let max_x = position.iter().map(|&[x, _]| x).max().unwrap();
  let min_y = position.iter().map(|&[_, y]| y).min().unwrap();
  let max_y = position.iter().map(|&[_, y]| y).max().unwrap();
  [[min_x, min_y], [max_x, max_y]]
}

fn draw(
  w: &mut dyn Write,
  corners: [Vec2; 2],
  position: &[Vec2],
) -> io::Result<()> {
  let set: HashSet<_> = position.iter().copied().collect();
  let [[min_x, min_y], [max_x, max_y]] = corners;
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      write!(w, "{}", block_char::get(set.contains(&[x, y])))?;
    }
    writeln!(w)?;
  }
  Ok(())
}

fn evolve(position: &mut [Vec2], velocity: &[Vec2]) {
  assert_eq!(position.len(), velocity.len());
  for ([x, y], &[dx, dy]) in position.iter_mut().zip(velocity.iter()) {
    *x += dx;
    *y += dy;
  }
}

const DRAW_THRESHOLD: isize = 100;

pub fn run(s: &str) -> io::Result<usize> {
  let (mut position, velocity) = parse(s);
  let mut stdout = std::io::stdout();
  let stdin = std::io::stdin();
  let mut buf = String::new();
  let mut counter = 0usize;
  loop {
    let [[min_x, min_y], [max_x, max_y]] = get_bounding_corners(&position);
    if max_x - min_x <= DRAW_THRESHOLD && max_y - min_y <= DRAW_THRESHOLD {
      draw(&mut stdout, [[min_x, min_y], [max_x, max_y]], &position)?;
      if stdin.read_line(&mut buf)? == 0 {
        return Ok(counter);
      }
      buf.clear();
    }
    evolve(&mut position, &velocity);
    counter += 1;
  }
}

#[test]
fn t() {
  let s = include_str!("input/d10.txt");
  let (mut position, velocity) = parse(s);
  let p2 = 10081usize;
  for _ in 0..p2 {
    evolve(&mut position, &velocity);
  }
  let mut buf = io::Cursor::new(Vec::new());
  draw(&mut buf, get_bounding_corners(&position), &position).unwrap();
  let p1 = r#"
░████░░░█████░░░█░░░░█░░█░░░░█░░██████░░██████░░█████░░░██████
█░░░░█░░█░░░░█░░█░░░░█░░█░░░█░░░█░░░░░░░░░░░░█░░█░░░░█░░░░░░░█
█░░░░░░░█░░░░█░░░█░░█░░░█░░█░░░░█░░░░░░░░░░░░█░░█░░░░█░░░░░░░█
█░░░░░░░█░░░░█░░░█░░█░░░█░█░░░░░█░░░░░░░░░░░█░░░█░░░░█░░░░░░█░
█░░░░░░░█████░░░░░██░░░░██░░░░░░█████░░░░░░█░░░░█████░░░░░░█░░
█░░░░░░░█░░█░░░░░░██░░░░██░░░░░░█░░░░░░░░░█░░░░░█░░░░░░░░░█░░░
█░░░░░░░█░░░█░░░░█░░█░░░█░█░░░░░█░░░░░░░░█░░░░░░█░░░░░░░░█░░░░
█░░░░░░░█░░░█░░░░█░░█░░░█░░█░░░░█░░░░░░░█░░░░░░░█░░░░░░░█░░░░░
█░░░░█░░█░░░░█░░█░░░░█░░█░░░█░░░█░░░░░░░█░░░░░░░█░░░░░░░█░░░░░
░████░░░█░░░░█░░█░░░░█░░█░░░░█░░██████░░██████░░█░░░░░░░██████
"#;
  let s = String::from_utf8(buf.into_inner()).unwrap();
  assert_eq!(s.trim(), p1.trim());
}
