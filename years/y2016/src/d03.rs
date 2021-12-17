type Tri = [u16; 3];

fn possible(tri: Tri) -> bool {
  let [a, b, c] = tri;
  a + b > c && a + c > b && b + c > a
}

fn parse(s: &str) -> impl Iterator<Item = Tri> + '_ {
  s.lines().map(|line| {
    let mut iter = line.split_ascii_whitespace();
    let a: u16 = iter.next().unwrap().parse().unwrap();
    let b: u16 = iter.next().unwrap().parse().unwrap();
    let c: u16 = iter.next().unwrap().parse().unwrap();
    [a, b, c]
  })
}

pub fn p1(s: &str) -> usize {
  parse(s).filter(|&tri| possible(tri)).count()
}

pub fn p2(s: &str) -> usize {
  let rows: Vec<_> = parse(s).collect();
  assert_eq!(rows.len() % 3, 0);
  let by_col: Vec<u16> = std::iter::empty()
    .chain(rows.iter().map(|&[a, _, _]| a))
    .chain(rows.iter().map(|&[_, b, _]| b))
    .chain(rows.iter().map(|&[_, _, c]| c))
    .collect();
  by_col
    .chunks_exact(3)
    .filter(|&w| possible(w.try_into().unwrap()))
    .count()
}

#[test]
fn t() {
  let s = include_str!("input/d03.txt");
  assert_eq!(p1(s), 917);
  assert_eq!(p2(s), 1649);
}
