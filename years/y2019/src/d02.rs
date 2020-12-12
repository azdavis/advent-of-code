pub fn p1(s: &str) -> usize {
  let ns = parse(s);
  go(ns, 12, 2)
}

pub fn p2(s: &str) -> usize {
  let ns = parse(s);
  for noun in 0..=99 {
    for verb in 0..=99 {
      if go(ns.clone(), noun, verb) == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  panic!()
}

fn go(mut ns: Vec<usize>, noun: usize, verb: usize) -> usize {
  ns[1] = noun;
  ns[2] = verb;
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

fn parse(s: &str) -> Vec<usize> {
  let mut lines = s.split('\n');
  let fst = lines.next().unwrap();
  fst.split(',').map(|s| s.parse().unwrap()).collect()
}
