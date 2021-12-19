use helpers::HashSet;

fn parse(s: &str) -> impl Iterator<Item = (Vec<&str>, Vec<&str>)> + '_ {
  s.lines().map(|mut s| {
    let mut outer = Vec::<&str>::default();
    let mut inner = Vec::<&str>::default();
    while let Some((outer_s, after)) = s.split_once('[') {
      let (inner_s, after_both) = after.split_once(']').unwrap();
      outer.push(outer_s);
      inner.push(inner_s);
      s = after_both;
    }
    outer.push(s);
    (outer, inner)
  })
}

fn any_has_abba(xs: &[&str]) -> bool {
  xs.iter().any(|s| {
    s.as_bytes()
      .windows(4)
      .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
  })
}

pub fn p1(s: &str) -> usize {
  parse(s)
    .filter(|(outer, inner)| any_has_abba(outer) && !any_has_abba(inner))
    .count()
}

fn get_aba<'a>(xs: &'a [&str]) -> impl Iterator<Item = [u8; 2]> + 'a {
  xs.iter().flat_map(|&s| {
    s.as_bytes()
      .windows(3)
      .filter_map(|w| (w[0] == w[2] && w[0] != w[1]).then(|| [w[0], w[1]]))
  })
}

pub fn p2(s: &str) -> usize {
  parse(s)
    .filter(|(outer, inner)| {
      let out_aba: HashSet<_> = get_aba(outer).collect();
      get_aba(inner).any(|[a, b]| out_aba.contains(&[b, a]))
    })
    .count()
}

#[test]
fn t() {
  let s = include_str!("input/d07.txt");
  assert_eq!(p1(s), 115);
  assert_eq!(p2(s), 231);
}
