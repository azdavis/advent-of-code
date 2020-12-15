pub fn p1(s: &str) -> i32 {
  let mut st = State::new();
  for ac in parse(s) {
    st.evolve(ac);
  }
  st.ship.to_origin()
}

pub fn p2(s: &str) -> i32 {
  let mut st = StateP2::new();
  for ac in parse(s) {
    st.evolve(ac);
  }
  st.ship.to_origin()
}

struct StateP2 {
  ship: Point,
  waypoint: Point,
}

impl StateP2 {
  fn new() -> Self {
    Self {
      ship: Point { x: 0, y: 0 },
      waypoint: Point { x: 10, y: 1 },
    }
  }

  fn evolve(&mut self, ac: Action) {
    match ac.kind {
      ActionKind::Direction(d) => self.waypoint.adjust(d, ac.num),
      ActionKind::Left => {
        assert_eq!(ac.num % 90, 0);
        for _ in 0..(ac.num / 90) % 4 {
          let old = self.waypoint;
          self.waypoint = Point {
            x: -old.y,
            y: old.x,
          };
        }
      }
      ActionKind::Right => {
        assert_eq!(ac.num % 90, 0);
        for _ in 0..(ac.num / 90) % 4 {
          let old = self.waypoint;
          self.waypoint = Point {
            x: old.y,
            y: -old.x,
          };
        }
      }
      ActionKind::Forward => {
        self.ship.x += self.waypoint.x * ac.num;
        self.ship.y += self.waypoint.y * ac.num;
      }
    }
  }
}

struct State {
  facing: Direction,
  ship: Point,
}

impl State {
  fn new() -> Self {
    Self {
      facing: Direction::East,
      ship: Point { x: 0, y: 0 },
    }
  }

  fn evolve(&mut self, ac: Action) {
    match ac.kind {
      ActionKind::Direction(d) => self.ship.adjust(d, ac.num),
      ActionKind::Left => {
        assert_eq!(ac.num % 90, 0);
        for _ in 0..(ac.num / 90) % 4 {
          self.facing = self.facing.left();
        }
      }
      ActionKind::Right => {
        assert_eq!(ac.num % 90, 0);
        for _ in 0..(ac.num / 90) % 4 {
          self.facing = self.facing.right();
        }
      }
      ActionKind::Forward => self.ship.adjust(self.facing, ac.num),
    }
  }
}

#[derive(Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn to_origin(&self) -> i32 {
    self.x.abs() + self.y.abs()
  }

  fn adjust(&mut self, d: Direction, n: i32) {
    match d {
      Direction::North => self.y += n,
      Direction::South => self.y -= n,
      Direction::East => self.x += n,
      Direction::West => self.x -= n,
    }
  }
}

fn parse(s: &str) -> impl Iterator<Item = Action> + '_ {
  s.split('\n')
    .filter(|line| !line.is_empty())
    .map(Action::parse)
}

struct Action {
  kind: ActionKind,
  num: i32,
}

impl Action {
  fn parse(s: &str) -> Action {
    let mut chars = s.chars();
    Action {
      kind: ActionKind::parse(chars.next().unwrap()),
      // as_str on Chars is nifty!
      num: chars.as_str().parse().unwrap(),
    }
  }
}

#[derive(Clone, Copy)]
enum Direction {
  North,
  South,
  East,
  West,
}

impl Direction {
  fn left(self) -> Self {
    match self {
      Self::North => Self::West,
      Self::South => Self::East,
      Self::East => Self::North,
      Self::West => Self::South,
    }
  }

  // yeah, it's just 3 lefts, but this saves some cycles.
  fn right(self) -> Self {
    match self {
      Self::North => Self::East,
      Self::South => Self::West,
      Self::East => Self::South,
      Self::West => Self::North,
    }
  }
}

enum ActionKind {
  Direction(Direction),
  Left,
  Right,
  Forward,
}

impl ActionKind {
  fn parse(c: char) -> Self {
    match c {
      'N' => Self::Direction(Direction::North),
      'S' => Self::Direction(Direction::South),
      'E' => Self::Direction(Direction::East),
      'W' => Self::Direction(Direction::West),
      'L' => Self::Left,
      'R' => Self::Right,
      'F' => Self::Forward,
      _ => panic!("bad action: {}", c),
    }
  }
}

#[test]
fn t() {
  let inp = include_str!("input/d12.txt");
  assert_eq!(p1(inp), 1133);
  assert_eq!(p2(inp), 61053);
}
