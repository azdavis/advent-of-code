use helpers::block_char;
use std::io::{self, Write};

enum Axis {
  X,
  Y,
}

enum Instr {
  Rect(usize, usize),
  Rotate(Axis, usize, usize),
}

fn parse(s: &str) -> impl Iterator<Item = Instr> + '_ {
  s.lines().map(|line| {
    let mut iter = line.split(' ');
    match iter.next().unwrap() {
      "rect" => {
        let dims = iter.next().unwrap();
        let (x, y) = dims.split_once('x').unwrap();
        Instr::Rect(x.parse().unwrap(), y.parse().unwrap())
      }
      "rotate" => {
        assert!(matches!(iter.next().unwrap(), "row" | "column"));
        let (axis, idx) = iter.next().unwrap().split_once('=').unwrap();
        assert_eq!(iter.next().unwrap(), "by");
        let axis = match axis {
          "x" => Axis::X,
          "y" => Axis::Y,
          it => panic!("unknown axis: {it}"),
        };
        let idx: usize = idx.parse().unwrap();
        let by: usize = iter.next().unwrap().parse().unwrap();
        Instr::Rotate(axis, idx, by)
      }
      it => panic!("unknown instr: {it}"),
    }
  })
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn run(s: &str) -> [[bool; WIDTH]; HEIGHT] {
  let mut screen = [[false; WIDTH]; HEIGHT];
  for instr in parse(s) {
    match instr {
      Instr::Rect(x, y) => {
        for row in screen.iter_mut().take(y) {
          for it in row.iter_mut().take(x) {
            *it = true;
          }
        }
      }
      Instr::Rotate(axis, idx, by) => match axis {
        Axis::X => {
          let mut col: Vec<_> = screen.iter().map(|row| row[idx]).collect();
          col.rotate_right(by);
          for (row, b) in screen.iter_mut().zip(col) {
            row[idx] = b;
          }
        }
        Axis::Y => screen[idx].rotate_right(by),
      },
    }
  }
  screen
}

pub fn p1(s: &str) -> usize {
  run(s).iter().flatten().filter(|&&it| it).count()
}

fn draw(w: &mut dyn Write, screen: &[[bool; WIDTH]; HEIGHT]) -> io::Result<()> {
  for row in screen {
    for &it in row {
      write!(w, "{}", block_char::get(it))?;
    }
    writeln!(w)?;
  }
  Ok(())
}

pub fn p2(s: &str) {
  let screen = run(s);
  draw(&mut io::stdout(), &screen).unwrap();
}

#[test]
fn t() {
  let s = include_str!("input/d08.txt");
  assert_eq!(p1(s), 115);
  let mut buf = io::Cursor::new(Vec::new());
  draw(&mut buf, &run(s)).unwrap();
  let p2 = r"
████░████░████░█░░░██░░█░████░███░░████░░███░░░██░
█░░░░█░░░░█░░░░█░░░██░█░░█░░░░█░░█░█░░░░░░█░░░░░█░
███░░███░░███░░░█░█░██░░░███░░█░░█░███░░░░█░░░░░█░
█░░░░█░░░░█░░░░░░█░░█░█░░█░░░░███░░█░░░░░░█░░░░░█░
█░░░░█░░░░█░░░░░░█░░█░█░░█░░░░█░█░░█░░░░░░█░░█░░█░
████░█░░░░████░░░█░░█░░█░█░░░░█░░█░█░░░░░███░░██░░
";
  let s = String::from_utf8(buf.into_inner()).unwrap();
  assert_eq!(s.trim(), p2.trim());
}
