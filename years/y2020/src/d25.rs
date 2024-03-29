pub fn p1(s: &str) -> u64 {
  let [pk1, pk2] = parse(s);
  let ls1 = get_loop_size(pk1);
  let ls2 = get_loop_size(pk2);
  transform(transform(INIT, ls1), ls2)
}

const INIT: u64 = 7;
const MOD: u64 = 20_201_227;

fn get_loop_size(pub_key: u64) -> u64 {
  let mut ret = 0;
  let mut cur = 1;
  loop {
    if cur == pub_key {
      return ret;
    }
    ret += 1;
    cur = (cur * INIT) % MOD;
  }
}

fn transform(subj: u64, loop_size: u64) -> u64 {
  let mut ret = 1;
  for _ in 0..loop_size {
    ret = (ret * subj) % MOD;
  }
  ret
}

fn parse(s: &str) -> [u64; 2] {
  let mut lines = s.lines();
  let pk1: u64 = lines.next().unwrap().parse().unwrap();
  let pk2: u64 = lines.next().unwrap().parse().unwrap();
  assert!(lines.next().is_none());
  [pk1, pk2]
}

#[test]
fn t() {
  let s = include_str!("input/d25.txt");
  assert_eq!(p1(s), 1_478_097);
}

#[test]
fn t_p1() {
  assert_eq!(p1("5764801\n17807724"), 14_897_079);
}
