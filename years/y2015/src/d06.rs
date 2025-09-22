enum InstrKind {
  TurnOn,
  TurnOff,
  Toggle,
}

type Coord = [usize; 2];

struct Instr {
  kind: InstrKind,
  top_left: Coord,
  bot_right: Coord,
}

fn parse_coord(s: &str) -> Coord {
  let (x, y) = s.split_once(',').unwrap();
  [x.parse().unwrap(), y.parse().unwrap()]
}

fn parse(s: &str) -> impl Iterator<Item = Instr> + '_ {
  s.lines().map(|line| {
    let mut iter = line.split_ascii_whitespace();
    let kind = match iter.next().unwrap() {
      "turn" => match iter.next().unwrap() {
        "on" => InstrKind::TurnOn,
        "off" => InstrKind::TurnOff,
        t => panic!("unknown turn kind: {t}"),
      },
      "toggle" => InstrKind::Toggle,
      t => panic!("unknown instr kind: {t}"),
    };
    let top_left = parse_coord(iter.next().unwrap());
    assert_eq!(iter.next(), Some("through"));
    let bot_right = parse_coord(iter.next().unwrap());
    assert!(iter.next().is_none());
    Instr {
      kind,
      top_left,
      bot_right,
    }
  })
}

const DIM: usize = 1000;

fn run<T>(
  s: &str,
  init: T,
  turn_on: fn(T) -> T,
  turn_off: fn(T) -> T,
  toggle: fn(T) -> T,
  to_n: fn(T) -> usize,
) -> usize
where
  T: Copy,
{
  let mut grid = vec![vec![init; DIM]; DIM];
  for instr in parse(s) {
    let f = match instr.kind {
      InstrKind::TurnOn => turn_on,
      InstrKind::TurnOff => turn_off,
      InstrKind::Toggle => toggle,
    };
    for row in &mut grid[instr.top_left[1]..=instr.bot_right[1]] {
      for tile in &mut row[instr.top_left[0]..=instr.bot_right[0]] {
        *tile = f(*tile);
      }
    }
  }
  grid.into_iter().flatten().map(to_n).sum()
}

pub fn p1(s: &str) -> usize {
  run(s, false, |_| true, |_| false, |x| !x, usize::from)
}

pub fn p2(s: &str) -> usize {
  run(
    s,
    0usize,
    |x| x + 1,
    |x| x.saturating_sub(1),
    |x| x + 2,
    std::convert::identity,
  )
}

#[test]
fn t() {
  let s = include_str!("input/d06.txt");
  assert_eq!(p1(s), 543_903);
  assert_eq!(p2(s), 14_687_245);
}
