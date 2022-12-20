use helpers::HashMap;

pub fn p1(s: &str) -> usize {
  go(s).into_values().sum()
}

pub fn p2(s: &str) -> usize {
  go(s).into_iter().min_by_key(|&(k, _)| k).unwrap().1
}

fn go(s: &str) -> HashMap<usize, usize> {
  let mut ns: Vec<_> = s.lines().map(|x| x.parse().unwrap()).collect();
  let mut ac = HashMap::<usize, usize>::default();
  go_rec(&mut ns, 150, 0, &mut ac);
  ac
}

fn go_rec(ns: &mut Vec<u8>, target: u8, used: usize, ac: &mut HashMap<usize, usize>) {
  let num = match ns.pop() {
    None => {
      if target == 0 {
        *ac.entry(used).or_default() += 1;
      }
      return;
    }
    Some(x) => x,
  };
  go_rec(ns, target, used, ac);
  if let Some(target) = target.checked_sub(num) {
    go_rec(ns, target, used + 1, ac);
  }
  ns.push(num);
}

#[test]
fn t() {
  let s = include_str!("input/d17.txt");
  assert_eq!(p1(s), 654);
  assert_eq!(p2(s), 57);
}
