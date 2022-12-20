use helpers::HashMap;

const EQ: [(&str, usize); 6] = [
  ("children", 3),
  ("samoyeds", 2),
  ("akitas", 0),
  ("vizslas", 0),
  ("cars", 2),
  ("perfumes", 1),
];

const GT: [(&str, usize); 2] = [("cats", 7), ("trees", 3)];

const LT: [(&str, usize); 2] = [("pomeranians", 3), ("goldfish", 5)];

fn run(s: &str, f: fn(HashMap<&str, usize>) -> bool) -> usize {
  let idx = s
    .lines()
    .map(|line| {
      let (_, info) = line.split_once(": ").unwrap();
      info
        .split(", ")
        .map(|part| {
          let (name, n) = part.split_once(": ").unwrap();
          (name, n.parse().unwrap())
        })
        .collect::<HashMap<&str, usize>>()
    })
    .position(f)
    .unwrap();
  idx + 1
}

fn has(map: &HashMap<&str, usize>, key: &str, val: usize, f: fn(&usize, &usize) -> bool) -> bool {
  map.get(key).map_or(true, |it| f(it, &val))
}

pub fn p1(s: &str) -> usize {
  run(s, |ref map| {
    std::iter::empty()
      .chain(EQ)
      .chain(GT)
      .chain(LT)
      .all(|(k, v)| has(map, k, v, PartialEq::eq))
  })
}

pub fn p2(s: &str) -> usize {
  run(s, |ref map| {
    EQ.into_iter().all(|(k, v)| has(map, k, v, PartialEq::eq))
      && GT.into_iter().all(|(k, v)| has(map, k, v, PartialOrd::gt))
      && LT.into_iter().all(|(k, v)| has(map, k, v, PartialOrd::lt))
  })
}

#[test]
fn t() {
  let s = include_str!("input/d16.txt");
  assert_eq!(p1(s), 213);
  assert_eq!(p2(s), 323);
}
