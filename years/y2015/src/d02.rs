type Dims = [u32; 3];

fn parse(s: &str) -> impl Iterator<Item = Dims> + '_ {
  s.lines().map(|line| {
    let mut parts = line.split('x');
    let l: u32 = parts.next().unwrap().parse().unwrap();
    let w: u32 = parts.next().unwrap().parse().unwrap();
    let h: u32 = parts.next().unwrap().parse().unwrap();
    assert!(parts.next().is_none());
    [l, w, h]
  })
}

pub fn p1(s: &str) -> u32 {
  parse(s)
    .map(|[l, w, h]| {
      let a = l * w;
      let b = w * h;
      let c = h * l;
      (a).min(b).min(c) + (2 * (a + b + c))
    })
    .sum()
}

pub fn p2(s: &str) -> u32 {
  parse(s)
    .map(|[l, w, h]| {
      let a = 2 * (l + w);
      let b = 2 * (w + h);
      let c = 2 * (h + l);
      a.min(b).min(c) + (l * w * h)
    })
    .sum()
}

#[test]
fn t() {
  let s = include_str!("input/d02.txt");
  assert_eq!(p1(s), 1_586_300);
  assert_eq!(p2(s), 3_737_498);
}
