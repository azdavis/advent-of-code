use crate::intcode::{Intcode, Res};
use std::io::{self, BufRead, Write};

pub fn run(s: &str) -> i64 {
  let stdin = io::stdin();
  let mut stdin = stdin.lock();
  let stdout = io::stdout();
  let mut stdout = stdout.lock();
  go(s, &mut stdin, &mut stdout)
}

fn go(s: &str, reader: &mut dyn BufRead, writer: &mut dyn Write) -> i64 {
  let mut prog = Intcode::parse(s);
  let mut buf_i64 = Vec::<i64>::new();
  let mut buf_u8 = Vec::<u8>::new();
  loop {
    let done = match prog.run(&mut buf_i64) {
      Res::Done => true,
      Res::NeedInput => false,
    };
    for n in buf_i64.drain(..) {
      buf_u8.push(n.try_into().unwrap());
    }
    writer.write_all(&buf_u8).unwrap();
    if done {
      let out = std::str::from_utf8(&buf_u8).unwrap();
      let (_, end) = out
        .split_once("You should be able to get in by typing ")
        .unwrap();
      let (n, _) = end.split_once(' ').unwrap();
      return n.parse().unwrap();
    }
    buf_u8.clear();
    reader.read_until(b'\n', &mut buf_u8).unwrap();
    for b in buf_u8.drain(..) {
      prog.input(b.try_into().unwrap());
    }
  }
}

#[test]
fn t() {
  let s = include_str!("input/d25.txt");
  let ans = include_str!("input/d25_sol.txt");
  let mut reader = std::io::Cursor::new(ans);
  let mut writer = std::io::sink();
  assert_eq!(go(s, &mut reader, &mut writer), 34095120);
}
