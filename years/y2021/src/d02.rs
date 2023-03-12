enum Dir {
  Forward,
  Up,
  Down,
}

fn parse(s: &str) -> impl Iterator<Item = (Dir, u32)> + '_ {
  s.lines().map(|line| {
    let (dir, num) = line.split_once(' ').unwrap();
    let dir = match dir {
      "forward" => Dir::Forward,
      "up" => Dir::Up,
      "down" => Dir::Down,
      _ => panic!("unknown dir: {dir}"),
    };
    (dir, num.parse().unwrap())
  })
}

pub fn p1(s: &str) -> u32 {
  let mut x = 0u32;
  let mut y = 0u32;
  for (dir, n) in parse(s) {
    match dir {
      Dir::Forward => x += n,
      Dir::Up => y -= n,
      Dir::Down => y += n,
    }
  }
  x * y
}

pub fn p2(s: &str) -> u32 {
  let mut x = 0u32;
  let mut y = 0u32;
  let mut aim = 0u32;
  for (dir, n) in parse(s) {
    match dir {
      Dir::Forward => {
        x += n;
        y += n * aim;
      }
      Dir::Up => aim -= n,
      Dir::Down => aim += n,
    }
  }
  x * y
}

#[test]
fn t() {
  let s = include_str!("input/d02.txt");
  assert_eq!(p1(s), 2_027_977);
  assert_eq!(p2(s), 1_903_644_897);
}
