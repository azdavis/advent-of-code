pub fn p1(s: &str) -> usize {
  let mut ns = parse(s);
  ns[1] = 12;
  ns[2] = 2;
  let mut idx = 0;
  while idx < ns.len() {
    match ns[idx] {
      1 => {
        let res = ns[idx + 3];
        ns[res] = ns[ns[idx + 1]] + ns[ns[idx + 2]];
      }
      2 => {
        let res = ns[idx + 3];
        ns[res] = ns[ns[idx + 1]] * ns[ns[idx + 2]];
      }
      99 => break,
      n => panic!("bad num: {}", n),
    }
    idx += 4;
  }
  ns[0]
}

pub fn p2(_: &str) -> usize {
  todo!()
}

fn parse(s: &str) -> Vec<usize> {
  let mut lines = s.split('\n');
  let fst = lines.next().unwrap();
  fst.split(',').map(|s| s.parse().unwrap()).collect()
}
