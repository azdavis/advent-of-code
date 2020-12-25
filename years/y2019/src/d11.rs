use crate::intcode::{Intcode, Res};
use helpers::compass::Compass;
use helpers::maplit::hashset;
use helpers::vec2::Vec2;
use std::collections::HashSet;

pub fn p1(s: &str) -> usize {
  go(s, &mut hashset![])
}

pub fn p2(s: &str) -> String {
  let mut white = hashset![Vec2::default()];
  go(s, &mut white);
  let min_x = white.iter().map(|p| p.x).min().unwrap();
  let min_y = white.iter().map(|p| p.y).min().unwrap();
  let max_x = white.iter().map(|p| p.x).max().unwrap();
  let max_y = white.iter().map(|p| p.y).max().unwrap();
  // too lazy to do with with_capacity calculation
  let mut ret = String::new();
  for y in (min_y..=max_y).rev() {
    for x in min_x..=max_x {
      let c = if white.contains(&Vec2::new(x, y)) {
        '█'
      } else {
        ' '
      };
      ret.push(c);
    }
    // the | prevents deleting trailing whitespace
    ret.push(' ');
    ret.push('|');
    ret.push('\n');
  }
  ret
}

fn go(s: &str, white: &mut HashSet<Vec2>) -> usize {
  let mut p = Intcode::parse(s);
  let mut did_paint = HashSet::<Vec2>::new();
  let mut cur = Vec2::default();
  let mut facing = Compass::North;
  let mut output = Vec::with_capacity(2);
  loop {
    match p.run(&mut output) {
      Res::Done => return did_paint.len(),
      Res::NeedInput => {}
    }
    match output[..] {
      [] => p.input(if white.contains(&cur) { 1 } else { 0 }),
      [color, dir] => {
        match color {
          0 => {
            white.remove(&cur);
          }
          1 => {
            white.insert(cur);
          }
          _ => panic!("bad color: {}", color),
        }
        did_paint.insert(cur);
        facing = match dir {
          0 => facing.left(),
          1 => facing.right(),
          _ => panic!("bad dir: {}", dir),
        };
        let [dx, dy] = facing.dx_dy();
        cur.x += dx;
        cur.y += dy;
        output.clear();
      }
      _ => panic!("bad output len: {}", output.len()),
    }
  }
}

#[test]
fn t() {
  let inp = include_str!("input/d11.txt");
  assert_eq!(p1(inp), 2160);
  let out = include_str!("snapshots/d11p2.txt");
  assert_eq!(p2(inp), out);
}
