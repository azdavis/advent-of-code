use helpers::{block_char, matrix::Coord, static_regex, HashSet};
use std::io::{self, Write};

type Coords = HashSet<Coord>;

static_regex!(FOLD = r"^fold along (\w)=(\d+)$");

#[derive(Debug, Clone, Copy)]
enum Axis {
  X,
  Y,
}

fn parse(s: &str) -> (Coords, Vec<(Axis, usize)>) {
  let mut lines = s.lines();
  let mut coords = Coords::default();
  loop {
    let line = lines.next().unwrap();
    if line.is_empty() {
      break;
    }
    let (x, y) = line.split_once(',').unwrap();
    coords.insert([x.parse().unwrap(), y.parse().unwrap()]);
  }
  let folds: Vec<_> = lines
    .map(|line| {
      let caps = FOLD.captures(line).unwrap();
      let axis = match &caps[1] {
        "x" => Axis::X,
        "y" => Axis::Y,
        d => panic!("unknown axis: {}", d),
      };
      let n: usize = caps[2].parse().unwrap();
      (axis, n)
    })
    .collect();
  (coords, folds)
}

fn execute_fold(coords: Coords, axis: Axis, on: usize) -> Coords {
  coords
    .into_iter()
    .map(|[x, y]| match axis {
      Axis::X => {
        if x < on {
          [x, y]
        } else {
          [on - (x - on), y]
        }
      }
      Axis::Y => {
        if y < on {
          [x, y]
        } else {
          [x, on - (y - on)]
        }
      }
    })
    .collect()
}

pub fn p1(s: &str) -> usize {
  let (coords, folds) = parse(s);
  let &(axis, on) = folds.first().unwrap();
  execute_fold(coords, axis, on).len()
}

fn p2_help(s: &str, w: &mut dyn Write) -> io::Result<()> {
  let (mut coords, folds) = parse(s);
  for (axis, on) in folds {
    coords = execute_fold(coords, axis, on);
  }
  // NOTE: basically taken from years/y2018/src/d10.rs
  let min_x = coords.iter().map(|&[x, _]| x).min().unwrap();
  let max_x = coords.iter().map(|&[x, _]| x).max().unwrap();
  let min_y = coords.iter().map(|&[_, y]| y).min().unwrap();
  let max_y = coords.iter().map(|&[_, y]| y).max().unwrap();
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      write!(w, "{}", block_char::get(coords.contains(&[x, y])))?;
    }
    writeln!(w)?;
  }
  Ok(())
}

pub fn p2(s: &str) -> io::Result<()> {
  let mut stdout = io::stdout();
  p2_help(s, &mut stdout)
}

#[test]
fn t() {
  let s = include_str!("input/d13.txt");
  assert_eq!(p1(s), 753);
  let p2_ans = r#"
█░░█░████░█░░░░████░█░░█░░░██░███░░█░░█
█░░█░░░░█░█░░░░█░░░░█░░█░░░░█░█░░█░█░█░
████░░░█░░█░░░░███░░████░░░░█░█░░█░██░░
█░░█░░█░░░█░░░░█░░░░█░░█░░░░█░███░░█░█░
█░░█░█░░░░█░░░░█░░░░█░░█░█░░█░█░█░░█░█░
█░░█░████░████░████░█░░█░░██░░█░░█░█░░█
"#;
  let mut w = io::Cursor::new(Vec::new());
  p2_help(s, &mut w).unwrap();
  let s = String::from_utf8(w.into_inner()).unwrap();
  assert_eq!(s.trim(), p2_ans.trim());
}
