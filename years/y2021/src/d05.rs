use helpers::HashMap;

type Coord = [u32; 2];
type Line = [Coord; 2];
type Counts = HashMap<Coord, usize>;

fn parse_coord(s: &str) -> Coord {
  let (x, y) = s.split_once(',').unwrap();
  [x.parse().unwrap(), y.parse().unwrap()]
}

fn parse(s: &str) -> impl Iterator<Item = Line> + '_ {
  s.lines().map(|line| {
    let (start, end) = line.split_once(" -> ").unwrap();
    [parse_coord(start), parse_coord(end)]
  })
}

fn mk_straight<F>(s: u32, e: u32, f: F) -> impl Iterator<Item = Coord>
where
  F: FnMut(u32) -> Coord,
{
  let min = s.min(e);
  let max = s.max(e);
  (min..=max).map(f)
}

fn add_points<I>(iter: I, counts: &mut Counts)
where
  I: Iterator<Item = Coord>,
{
  for coord in iter {
    *counts.entry(coord).or_default() += 1
  }
}

fn add_straight(line: Line, counts: &mut Counts) -> bool {
  let [[sx, sy], [ex, ey]] = line;
  if sx == ex {
    add_points(mk_straight(sy, ey, |y| [sx, y]), counts);
    true
  } else if sy == ey {
    add_points(mk_straight(sx, ex, |x| [x, sy]), counts);
    true
  } else {
    false
  }
}

fn run<F>(s: &str, f: &mut F) -> usize
where
  F: FnMut(Line, &mut Counts),
{
  let mut counts = Counts::default();
  for line in parse(s) {
    f(line, &mut counts);
  }
  counts.values().filter(|&&x| x > 1).count()
}

pub fn p1(s: &str) -> usize {
  run(s, &mut |line, counts| {
    add_straight(line, counts);
  })
}

pub fn p2(s: &str) -> usize {
  run(s, &mut |line, counts| {
    if add_straight(line, counts) {
      return;
    }
    let [[sx, sy], [ex, ey]] = line;
    let (sx, sy, ex, ey) = if sx < ex {
      (sx, sy, ex, ey)
    } else {
      (ex, ey, sx, sy)
    };
    add_points(
      (sx..=ex).enumerate().map(|(idx, x)| {
        let dy = u32::try_from(idx).unwrap();
        let y = if sy < ey { sy + dy } else { sy - dy };
        [x, y]
      }),
      counts,
    )
  })
}

#[test]
fn t() {
  let s = include_str!("input/d05.txt");
  assert_eq!(p1(s), 6710);
  assert_eq!(p2(s), 20121);
}

#[test]
fn ex1() {
  let s = include_str!("input/d05_ex1.txt");
  assert_eq!(p1(s), 5);
  assert_eq!(p2(s), 12);
}
