use helpers::{Compass, Vec2};

pub fn p1(s: &str) -> u32 {
  let mut st = State::new();
  for ac in parse(s) {
    st.evolve(ac);
  }
  st.ship.to_origin()
}

pub fn p2(s: &str) -> u32 {
  let mut st = StateP2::new();
  for ac in parse(s) {
    st.evolve(ac);
  }
  st.ship.to_origin()
}

struct StateP2 {
  ship: Vec2,
  waypoint: Vec2,
}

impl StateP2 {
  fn new() -> Self {
    Self {
      ship: Vec2::new(0, 0),
      waypoint: Vec2::new(10, 1),
    }
  }

  fn evolve(&mut self, ac: Action) {
    match ac.kind {
      ActionKind::Compass(d) => adjust(&mut self.waypoint, d, ac.num),
      ActionKind::Left => {
        assert_eq!(ac.num % 90, 0);
        for _ in 0..(ac.num / 90) % 4 {
          let old = self.waypoint;
          self.waypoint.x = -old.y;
          self.waypoint.y = old.x;
        }
      }
      ActionKind::Right => {
        assert_eq!(ac.num % 90, 0);
        for _ in 0..(ac.num / 90) % 4 {
          let old = self.waypoint;
          self.waypoint.x = old.y;
          self.waypoint.y = -old.x;
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
  ship: Vec2,
}

impl State {
  fn new() -> Self {
    Self {
      facing: Compass::East,
      ship: Vec2::default(),
    }
  }

  fn evolve(&mut self, ac: Action) {
    match ac.kind {
      ActionKind::Compass(d) => adjust(&mut self.ship, d, ac.num),
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
      ActionKind::Forward => adjust(&mut self.ship, self.facing, ac.num),
    }
  }
}

fn adjust(p: &mut Vec2, d: Compass, n: i32) {
  match d {
    Compass::North => p.y += n,
    Compass::South => p.y -= n,
    Compass::East => p.x += n,
    Compass::West => p.x -= n,
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
  let s = include_str!("input/d12.txt");
  assert_eq!(p1(s), 1133);
  assert_eq!(p2(s), 61053);
}
