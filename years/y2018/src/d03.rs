use helpers::static_regex;

type Vec2 = [usize; 2];

struct Claim {
  id: usize,
  pos: Vec2,
  dim: Vec2,
}

const DIM: usize = 1000;
static_regex!(RE = r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$");

fn parse(s: &str) -> Vec<Claim> {
  s.lines()
    .map(|line| {
      let caps = RE.captures(line).unwrap();
      Claim {
        id: caps[1].parse().unwrap(),
        pos: [caps[2].parse().unwrap(), caps[3].parse().unwrap()],
        dim: [caps[4].parse().unwrap(), caps[5].parse().unwrap()],
      }
    })
    .collect()
}

fn run(claims: &[Claim]) -> Vec<Vec<usize>> {
  let mut grid = vec![vec![0usize; DIM]; DIM];
  for cl in claims {
    let [x, y] = cl.pos;
    let [dx, dy] = cl.dim;
    for row in grid.iter_mut().skip(y).take(dy) {
      for n in row.iter_mut().skip(x).take(dx) {
        *n += 1;
      }
    }
  }
  grid
}

pub fn p1(s: &str) -> usize {
  let grid = run(&parse(s));
  grid.into_iter().flatten().filter(|&n| n > 1).count()
}

pub fn p2(s: &str) -> usize {
  let claims = parse(s);
  let grid = run(&claims);
  for cl in claims {
    let [x, y] = cl.pos;
    let [dx, dy] = cl.dim;
    let all_1 = grid
      .iter()
      .skip(y)
      .take(dy)
      .flat_map(|row| row.iter().skip(x).take(dx))
      .all(|&n| n == 1);
    if all_1 {
      return cl.id;
    }
  }
  panic!("no solution")
}

#[test]
fn t() {
  let s = include_str!("input/d03.txt");
  assert_eq!(p1(s), 101_565);
  assert_eq!(p2(s), 656);
}
