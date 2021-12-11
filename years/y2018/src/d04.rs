use helpers::{static_regex, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
  year: u16,
  month: u8,
  day: u8,
  hour: u8,
  minute: u8,
}

#[derive(Debug, Clone, Copy)]
enum Action {
  BeginShift(usize),
  Wake,
  Sleep,
}

static_regex!(LINE = r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.+)$");
static_regex!(BEGIN_SHIFT = r"^Guard #(\d+) begins shift$");

fn parse(s: &str) -> Vec<(Date, Action)> {
  s.lines()
    .map(|line| {
      let caps = LINE.captures(line).unwrap();
      let date = Date {
        year: caps[1].parse().unwrap(),
        month: caps[2].parse().unwrap(),
        day: caps[3].parse().unwrap(),
        hour: caps[4].parse().unwrap(),
        minute: caps[5].parse().unwrap(),
      };
      let action = match &caps[6] {
        "wakes up" => Action::Wake,
        "falls asleep" => Action::Sleep,
        s => {
          let caps = BEGIN_SHIFT.captures(s).unwrap();
          Action::BeginShift(caps[1].parse().unwrap())
        }
      };
      (date, action)
    })
    .collect()
}

const HR_MINS: usize = 60;

#[allow(clippy::needless_range_loop)]
fn guard_sleep(s: &str) -> HashMap<usize, Vec<usize>> {
  let mut log = parse(s);
  log.sort_unstable_by_key(|e| e.0);
  let mut log = log.into_iter();
  let (date, action) = log.next().unwrap();
  let mut minute = usize::from(date.minute);
  let mut on_duty = match action {
    Action::BeginShift(g) => g,
    Action::Wake | Action::Sleep => unreachable!(),
  };
  let mut ret = HashMap::<usize, Vec<usize>>::default();
  for (date, action) in log {
    let new_minute = usize::from(date.minute);
    match action {
      Action::BeginShift(g) => on_duty = g,
      Action::Wake => {
        let entry = ret.entry(on_duty).or_insert_with(|| vec![0; HR_MINS]);
        for m in minute..new_minute {
          entry[m] += 1;
        }
      }
      Action::Sleep => {}
    }
    minute = new_minute;
  }
  ret
}

pub fn p1(s: &str) -> usize {
  let gs = guard_sleep(s);
  let (guard, entry) = gs
    .into_iter()
    .max_by_key(|(_, entry)| entry.iter().sum::<usize>())
    .unwrap();
  let (minute, _) = entry
    .into_iter()
    .enumerate()
    .max_by_key(|&(_, n)| n)
    .unwrap();
  guard * minute
}

pub fn p2(s: &str) -> usize {
  let gs = guard_sleep(s);
  let (guard, minute, _) = gs
    .into_iter()
    .map(|(g, entry)| {
      let (m, n) = entry
        .into_iter()
        .enumerate()
        .max_by_key(|&(_, n)| n)
        .unwrap();
      (g, m, n)
    })
    .max_by_key(|&(_, _, n)| n)
    .unwrap();
  guard * minute
}

#[test]
fn t() {
  let s = include_str!("input/d04.txt");
  assert_eq!(p1(s), 60438);
  assert_eq!(p2(s), 47989);
}
