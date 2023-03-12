use helpers::Counter;

type Coord = [u32; 2];
type Line = [Coord; 2];
type Counts = Counter<Coord>;

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

fn straight(line: Line) -> Option<Box<dyn Iterator<Item = Coord>>> {
  let [[sx, sy], [ex, ey]] = line;
  if sx == ex {
    Some(Box::new(mk_straight(sy, ey, move |y| [sx, y])))
  } else if sy == ey {
    Some(Box::new(mk_straight(sx, ex, move |x| [x, sy])))
  } else {
    None
  }
}

fn run<F, I>(s: &str, f: &mut F) -> usize
where
  F: FnMut(Line) -> I,
  I: Iterator<Item = Coord>,
{
  let mut counts = Counts::default();
  for line in parse(s) {
    for coord in f(line) {
      counts.inc(coord);
    }
  }
  counts.iter().filter(|&(_, x)| x > 1).count()
}

pub fn p1(s: &str) -> usize {
  run(s, &mut |line| straight(line).into_iter().flatten())
}

pub fn p2(s: &str) -> usize {
  run(s, &mut |line| {
    straight(line).unwrap_or_else(|| {
      let [[sx, sy], [ex, ey]] = line;
      let (sx, sy, ex, ey) = if sx < ex {
        (sx, sy, ex, ey)
      } else {
        (ex, ey, sx, sy)
      };
      Box::new((sx..=ex).enumerate().map(move |(idx, x)| {
        let dy = u32::try_from(idx).unwrap();
        let y = if sy < ey { sy + dy } else { sy - dy };
        [x, y]
      }))
    })
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
