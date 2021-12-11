use helpers::matrix::neighbors;

pub fn p1(s: &str) -> u32 {
  let grid: Vec<Vec<u8>> = s
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect()
    })
    .collect();
  grid
    .iter()
    .enumerate()
    .map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .filter_map(|(x, &n1)| {
          neighbors(&grid, [x, y])
            .all(|(&n2, _)| n1 < n2)
            .then(|| u32::from(n1) + 1)
        })
        .sum::<u32>()
    })
    .sum()
}

pub fn p2(s: &str) -> usize {
  // TODO
  s.len()
}

#[test]
fn t() {
  let s = include_str!("input/d09.txt");
  assert_eq!(p1(s), 539);
  // assert_eq!(p2(s), 0);
}
