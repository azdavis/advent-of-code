pub fn p1(s: &str) -> u32 {
  let inp = parse(s);
  let start = inp.start;
  let (id, wait) = inp
    .buses
    .into_iter()
    .map(|(_, x)| (x, wait_time(start, x)))
    .min_by_key(|x| x.1)
    .unwrap();
  id * wait
}

pub fn p2(_: &str) -> u32 {
  todo!()
}

fn wait_time(start: u32, id: u32) -> u32 {
  let r = start % id;
  if r == 0 {
    0
  } else {
    id - r
  }
}

struct Input {
  start: u32,
  buses: Vec<(usize, u32)>,
}

fn parse(s: &str) -> Input {
  let mut lines = s.split('\n');
  Input {
    start: lines.next().unwrap().parse().unwrap(),
    buses: lines
      .next()
      .unwrap()
      .split(',')
      .enumerate()
      .filter(|&(_, x)| x != "x")
      .map(|(idx, x)| (idx, x.parse().unwrap()))
      .collect(),
  }
}
