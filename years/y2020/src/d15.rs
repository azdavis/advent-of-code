use std::collections::HashMap;

pub fn p1(s: &str) -> usize {
  go(s, 2020)
}

pub fn p2(s: &str) -> usize {
  go(s, 30000000)
}

fn go(s: &str, target: usize) -> usize {
  let nums = parse(s);
  let mut turn = 1;
  let mut map = Map::new();
  for &num in nums.iter() {
    update(&mut map, num, turn);
    turn += 1;
  }
  let mut cur = *nums.last().unwrap();
  loop {
    let info = map.get(&cur).unwrap();
    cur = match info.snd {
      None => 0,
      Some(x) => info.fst - x,
    };
    update(&mut map, cur, turn);
    if turn == target {
      return cur;
    }
    turn += 1;
  }
}

type Map = HashMap<usize, Info>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Info {
  fst: usize,
  snd: Option<usize>,
}

fn update(map: &mut Map, num: usize, fst: usize) {
  let snd = map.get(&num).map(|x| x.fst);
  map.insert(num, Info { fst, snd });
}

fn parse(s: &str) -> Vec<usize> {
  s.split('\n')
    .next()
    .unwrap()
    .split(',')
    .map(|x| x.parse().unwrap())
    .collect()
}

#[test]
fn t_p1() {
  assert_eq!(p1("0,3,6\n"), 436);
  assert_eq!(p1("1,3,2\n"), 1);
  assert_eq!(p1("2,1,3\n"), 10);
  assert_eq!(p1("1,2,3\n"), 27);
  assert_eq!(p1("2,3,1\n"), 78);
  assert_eq!(p1("3,2,1\n"), 438);
  assert_eq!(p1("3,1,2\n"), 1836);
}

#[test]
fn t() {
  let inp = include_str!("input/d15.txt");
  assert_eq!(p1(inp), 1696);
  assert_eq!(p2(inp), 37385);
}
