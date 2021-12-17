use helpers::static_regex;

static_regex!(RE = r"^target area: x=(\d+)..(\d+), y=-(\d+)..-(\d+)$");

type Bounds = [[isize; 2]; 2];

fn parse(s: &str) -> Bounds {
  let caps = RE.captures(s.trim()).unwrap();
  let x_min: isize = caps[1].parse().unwrap();
  let x_max: isize = caps[2].parse().unwrap();
  let y_min: isize = caps[3].parse().unwrap();
  let y_max: isize = caps[4].parse().unwrap();
  [[x_min, x_max], [-y_min, -y_max]]
}

fn dy_max(y_min: isize) -> isize {
  -y_min - 1
}

pub fn p1(s: &str) -> isize {
  let [_, [y_min, _]] = parse(s);
  let mut dy = dy_max(y_min);
  let mut ret = 0isize;
  let mut y = 0isize;
  while dy > 0 {
    y += dy;
    dy -= 1;
    ret = ret.max(y);
  }
  ret
}

fn enters_bounds(bounds: Bounds, mut dx: isize, mut dy: isize) -> bool {
  assert!(dx > 0);
  let [[x_min, x_max], [y_min, y_max]] = bounds;
  let x_bounds = x_min..=x_max;
  let y_bounds = y_min..=y_max;
  let mut x = 0isize;
  let mut y = 0isize;
  loop {
    x += dx;
    y += dy;
    if dx > 0 {
      dx -= 1;
    }
    dy -= 1;
    if x_bounds.contains(&x) && y_bounds.contains(&y) {
      return true;
    }
    if x > x_max || y < y_min {
      return false;
    }
  }
}

pub fn p2(s: &str) -> usize {
  let bounds @ [[_, x_max], [y_min, _]] = parse(s);
  (1isize..=x_max)
    .flat_map(|dx| {
      (y_min..=dy_max(y_min)).filter(move |&dy| enters_bounds(bounds, dx, dy))
    })
    .count()
}

#[test]
fn t() {
  let s = include_str!("input/d17.txt");
  assert_eq!(p1(s), 5565);
  assert_eq!(p2(s), 2118);
}

#[test]
fn ex1() {
  let s = "target area: x=20..30, y=-10..-5";
  assert_eq!(p1(s), 45);
  assert_eq!(p2(s), 112);
}
