pub fn p1(s: &str) -> usize {
  let inp = parse(s);
  let start = inp.start;
  let (wait, bus) = inp
    .buses
    .into_iter()
    .map(|bus| (wait_time(start, bus.id), bus))
    .min_by_key(|x| x.0)
    .unwrap();
  bus.id * wait
}

pub fn p2(_: &str) -> usize {
  todo!()
}

fn wait_time(start: usize, id: usize) -> usize {
  let r = start % id;
  if r == 0 {
    0
  } else {
    id - r
  }
}

struct Input {
  start: usize,
  buses: Vec<Bus>,
}

struct Bus {
  idx: usize,
  id: usize,
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
      .map(|(idx, x)| Bus {
        idx,
        id: x.parse().unwrap(),
      })
      .collect(),
  }
}
