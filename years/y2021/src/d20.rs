use helpers::HashMap;

fn is_lit(c: char) -> bool {
  match c {
    '#' => true,
    '.' => false,
    _ => panic!("unknown pixel: {}", c),
  }
}

type Coord = [isize; 2];

fn parse(s: &str) -> (Vec<bool>, HashMap<Coord, bool>) {
  let mut lines = s.lines();
  let alg: Vec<_> = lines.next().unwrap().chars().map(is_lit).collect();
  assert!(lines.next().unwrap().is_empty());
  let set: HashMap<Coord, _> = lines
    .enumerate()
    .flat_map(|(y, line)| {
      line.chars().enumerate().map(move |(x, c)| {
        ([x.try_into().unwrap(), y.try_into().unwrap()], is_lit(c))
      })
    })
    .collect();
  (alg, set)
}

fn neighbors_and_self(coord: Coord) -> [Coord; 9] {
  let [x, y] = coord;
  [
    [x - 1, y - 1],
    [x, y - 1],
    [x + 1, y - 1],
    [x - 1, y],
    [x, y],
    [x + 1, y],
    [x - 1, y + 1],
    [x, y + 1],
    [x + 1, y + 1],
  ]
}

fn run(s: &str, n: usize) -> usize {
  assert_eq!(n % 2, 0);
  let (alg, mut img) = parse(s);
  let &inf_lit_when_odd = alg.first().unwrap();
  for idx in 0..n {
    let min_x = img.keys().map(|&[x, _]| x).min().unwrap() - 1;
    let max_x = img.keys().map(|&[x, _]| x).max().unwrap() + 1;
    let min_y = img.keys().map(|&[_, y]| y).min().unwrap() - 1;
    let max_y = img.keys().map(|&[_, y]| y).max().unwrap() + 1;
    img = (min_y..=max_y)
      .flat_map(|y| (min_x..=max_x).map(move |x| [x, y]))
      .map(|coord| {
        let idx = neighbors_and_self(coord)
          .into_iter()
          .map(|c| {
            img
              .get(&c)
              .copied()
              .unwrap_or(idx % 2 == 1 && inf_lit_when_odd)
          })
          .fold(0usize, |ac, x| (ac << 1) | usize::from(x));
        (coord, alg[idx])
      })
      .collect();
  }
  img.values().filter(|&&it| it).count()
}

pub fn p1(s: &str) -> usize {
  run(s, 2)
}

pub fn p2(s: &str) -> usize {
  run(s, 50)
}

#[test]
fn t() {
  let s = include_str!("input/d20.txt");
  assert_eq!(p1(s), 5306);
  assert_eq!(p2(s), 17497);
}
