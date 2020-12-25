use helpers::compass::Compass;

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
      ActionKind::Compass(d) => self.waypoint.adjust(d, ac.num),
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
  facing: Compass,
  ship: Point,
}

impl State {
  fn new() -> Self {
    Self {
      facing: Compass::East,
      ship: Point { x: 0, y: 0 },
    }
  }

  fn evolve(&mut self, ac: Action) {
    match ac.kind {
      ActionKind::Compass(d) => self.ship.adjust(d, ac.num),
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

  fn adjust(&mut self, d: Compass, n: i32) {
    match d {
      Compass::North => self.y += n,
      Compass::South => self.y -= n,
      Compass::East => self.x += n,
      Compass::West => self.x -= n,
    }
  }
}

fn parse(s: &str) -> impl Iterator<Item = Action> + '_ {
  s.lines().map(Action::parse)
}

struct Action {
  kind: ActionKind,
  num: i32,
}

impl Action {
  fn parse(s: &str) -> Self {
    let mut chars = s.chars();
    Self {
      kind: ActionKind::parse(chars.next().unwrap()),
      // as_str on Chars is nifty!
      num: chars.as_str().parse().unwrap(),
    }
  }
}

enum ActionKind {
  Compass(Compass),
  Left,
  Right,
  Forward,
}

impl ActionKind {
  fn parse(c: char) -> Self {
    match c {
      'N' => Self::Compass(Compass::North),
      'S' => Self::Compass(Compass::South),
      'E' => Self::Compass(Compass::East),
      'W' => Self::Compass(Compass::West),
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
