use helpers::static_regex;

static_regex!(
  RE = r"^\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$"
);

#[derive(Debug, Clone, Copy)]
struct State {
  distance: u32,
  counter: u32,
  kind: Kind,
  points: u32,
}

impl State {
  fn new(counter: u32) -> State {
    State {
      distance: 0,
      counter,
      kind: Kind::Run,
      points: 0,
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Kind {
  Run,
  Rest,
}

struct Rule {
  speed: u32,
  run_duration: u32,
  rest_duration: u32,
}

const DURATION: u32 = 2503;

fn simulate(s: &str, f: fn(State) -> u32) -> u32 {
  let rules: Vec<_> = s
    .lines()
    .map(|line| {
      let caps = RE.captures(line).unwrap();
      Rule {
        speed: caps.get(1).unwrap().as_str().parse().unwrap(),
        run_duration: caps.get(2).unwrap().as_str().parse().unwrap(),
        rest_duration: caps.get(3).unwrap().as_str().parse().unwrap(),
      }
    })
    .collect();
  let mut states: Vec<_> = rules
    .iter()
    .map(|rule| State::new(rule.run_duration))
    .collect();
  for _ in 0..DURATION {
    for (st, rule) in states.iter_mut().zip(rules.iter()) {
      st.counter -= 1;
      match st.kind {
        Kind::Run => st.distance += rule.speed,
        Kind::Rest => {}
      }
      if st.counter == 0 {
        let (kind, counter) = match st.kind {
          Kind::Run => (Kind::Rest, rule.rest_duration),
          Kind::Rest => (Kind::Run, rule.run_duration),
        };
        st.kind = kind;
        st.counter = counter;
      }
    }
    let max_dist = states.iter().map(|st| st.distance).max().unwrap();
    for st in &mut states {
      if st.distance == max_dist {
        st.points += 1;
      }
    }
  }
  states.into_iter().map(f).max().unwrap()
}

pub fn p1(s: &str) -> u32 {
  simulate(s, |it| it.distance)
}

pub fn p2(s: &str) -> u32 {
  simulate(s, |it| it.points)
}

#[test]
fn t() {
  let s = include_str!("input/d14.txt");
  assert_eq!(p1(s), 2640);
  assert_eq!(p2(s), 1102);
}
