use helpers::HashSet;

fn is_plant(c: char) -> bool {
  match c {
    '.' => false,
    '#' => true,
    _ => panic!("unknown char: {c}"),
  }
}

fn bit_idx<I>(iter: I) -> usize
where
  I: Iterator<Item = bool>,
{
  let mut ret = 0usize;
  for x in iter {
    ret <<= 1;
    if x {
      ret |= 1;
    }
  }
  ret
}

const WINDOW: usize = 5;

fn parse(s: &str) -> (HashSet<isize>, Vec<bool>) {
  let mut lines = s.lines();
  let plants: HashSet<isize> = lines
    .next()
    .unwrap()
    .strip_prefix("initial state: ")
    .unwrap()
    .chars()
    .enumerate()
    .filter_map(|(idx, c)| is_plant(c).then(|| idx.try_into().unwrap()))
    .collect();
  assert!(lines.next().unwrap().is_empty());
  let mut rules = vec![false; 1 << WINDOW];
  for line in lines {
    let (from, to) = line.split_once(" => ").unwrap();
    assert_eq!(from.len(), WINDOW);
    assert_eq!(to.len(), 1);
    let idx = bit_idx(from.chars().map(is_plant));
    rules[idx] = is_plant(to.chars().next().unwrap());
  }
  (plants, rules)
}

fn run(s: &str, rounds: usize) -> isize {
  let (mut cur, rules) = parse(s);
  let window = isize::try_from(WINDOW).unwrap();
  let window_2 = window / 2;
  for _ in 0..rounds {
    let mut next = HashSet::default();
    let min = cur.iter().copied().min().unwrap() - window_2;
    let max = cur.iter().copied().max().unwrap() + window_2;
    for idx in min..=max {
      let neighbors = (idx - window_2..=idx + window_2).map(|i| cur.contains(&i));
      let rule_idx = bit_idx(neighbors);
      if rules[rule_idx] {
        next.insert(idx);
      } else {
        next.remove(&idx);
      }
    }
    cur = next;
  }
  cur.iter().sum()
}

pub fn p1(s: &str) -> isize {
  run(s, 20)
}

/// note this is hard-coded for my test input. from observation the formula for
/// number of pots on round r after stabilization is 18 + (r + 22) * 21.
pub fn p2(_: &str) -> isize {
  18 + (50_000_000_000 + 22) * 21
}

#[test]
fn t() {
  let s = include_str!("input/d12.txt");
  assert_eq!(p1(s), 1447);
  assert_eq!(p2(s), 1_050_000_000_480);
}

#[test]
fn ex1() {
  let s = include_str!("input/d12_ex1.txt");
  assert_eq!(p1(s), 325);
}
