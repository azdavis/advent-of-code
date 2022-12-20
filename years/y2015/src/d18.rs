use helpers::neighbors::neighbors_diag;

pub fn p1(s: &str) -> usize {
  go(s, false)
}

pub fn p2(s: &str) -> usize {
  go(s, true)
}

fn go(s: &str, corners_on: bool) -> usize {
  let mut grid: Vec<Vec<_>> = s
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          '#' => true,
          '.' => false,
          _ => panic!("unknown char: {c}"),
        })
        .collect()
    })
    .collect();
  let w = grid.first().unwrap().len() - 1;
  let h = grid.len() - 1;
  let corners = [[0, 0], [0, h], [w, 0], [w, h]];
  for _ in 0..100 {
    grid = grid
      .iter()
      .enumerate()
      .map(|(y, row)| {
        row
          .iter()
          .enumerate()
          .map(|(x, &on)| {
            let coord = [x, y];
            if corners_on && corners.contains(&coord) {
              return true;
            }
            let on_neighbors = neighbors_diag(&grid, coord).filter(|(&on, _)| on).count();
            if on {
              matches!(on_neighbors, 2 | 3)
            } else {
              on_neighbors == 3
            }
          })
          .collect()
      })
      .collect();
  }
  grid.iter().flatten().filter(|&&on| on).count()
}

#[test]
fn t() {
  let s = include_str!("input/d18.txt");
  assert_eq!(p1(s), 768);
  assert_eq!(p2(s), 781);
}
