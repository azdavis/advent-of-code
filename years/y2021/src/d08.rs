use helpers::{hash_set, HashMap, HashSet};

pub fn p1(s: &str) -> usize {
  s.lines()
    .flat_map(|line| {
      let (_, tail) = line.split_once(" | ").unwrap();
      tail.split(' ')
    })
    .filter(|&x| matches!(x.len(), 2 | 3 | 4 | 7))
    .count()
}

type Counts = HashMap<char, u8>;

fn get_count(counts: &Counts, want: u8) -> impl Iterator<Item = char> + '_ {
  counts
    .iter()
    .filter_map(move |(&c, &count)| (count == want).then(|| c))
}

fn get_digit<F>(digits: &[&str], len: usize, pred: F) -> char
where
  F: FnMut(&char) -> bool,
{
  digits
    .iter()
    .find(|&&def| def.len() == len)
    .unwrap()
    .chars()
    .find(pred)
    .unwrap()
}

fn get_one_line(line: &str) -> usize {
  let (digits, num) = line.split_once(" | ").unwrap();
  let mut counts: Counts = ('a'..='g').map(|c| (c, 0u8)).collect();
  for c in digits.chars() {
    if let Some(count) = counts.get_mut(&c) {
      *count += 1;
    }
  }
  let seg_e = get_count(&counts, 4).next().unwrap();
  let seg_b = get_count(&counts, 6).next().unwrap();
  let seg_f = get_count(&counts, 9).next().unwrap();
  let digits: Vec<_> = digits.split(' ').collect();
  let seg_c = get_digit(&digits, 2, |&c| c != seg_f);
  let seg_a = get_count(&counts, 8).find(|&c| c != seg_c).unwrap();
  let seg_d =
    get_digit(&digits, 4, |&c| c != seg_b && c != seg_c && c != seg_f);
  let seg_g = get_count(&counts, 7).find(|&c| c != seg_d).unwrap();
  let mapping = [
    hash_set([seg_a, seg_b, seg_c, seg_e, seg_f, seg_g]),
    hash_set([seg_c, seg_f]),
    hash_set([seg_a, seg_c, seg_d, seg_e, seg_g]),
    hash_set([seg_a, seg_c, seg_d, seg_f, seg_g]),
    hash_set([seg_b, seg_c, seg_d, seg_f]),
    hash_set([seg_a, seg_b, seg_d, seg_f, seg_g]),
    hash_set([seg_a, seg_b, seg_d, seg_e, seg_f, seg_g]),
    hash_set([seg_a, seg_c, seg_f]),
    hash_set([seg_a, seg_b, seg_c, seg_d, seg_e, seg_f, seg_g]),
    hash_set([seg_a, seg_b, seg_c, seg_d, seg_f, seg_g]),
  ];
  num
    .split(' ')
    .map(|digit| {
      let digits: HashSet<_> = digit.chars().collect();
      mapping
        .iter()
        .enumerate()
        .find_map(|(idx, ds)| (digits == *ds).then(|| idx))
        .unwrap()
    })
    .fold(0usize, |ac, x| (ac * 10) + x)
}

pub fn p2(s: &str) -> usize {
  s.lines().map(get_one_line).sum()
}

#[test]
fn t() {
  let s = include_str!("input/d08.txt");
  assert_eq!(p1(s), 479);
  assert_eq!(p2(s), 1041746);
}
