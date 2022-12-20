use helpers::{hash_set, HashMap, HashSet};

pub fn p1(s: &str) -> usize {
  let (g, target) = parse(s);
  let mut ac = HashSet::<String>::default();
  replace_all(&g, target, &mut |s| {
    ac.insert(s);
  });
  ac.len()
}

/// takes way too long. using a visited set doesn't help either.
pub fn p2(s: &str) -> usize {
  let (g, target) = parse(s);
  let mut rev_g = Graph::<&str>::default();
  for (k, vs) in g {
    for v in vs {
      rev_g.entry(v).or_default().insert(k);
    }
  }
  let mut rounds = 0usize;
  let mut ac = hash_set([target.to_owned()]);
  loop {
    let mut new_ac = HashSet::<String>::default();
    for start in ac {
      replace_all(&rev_g, start.as_str(), &mut |s| {
        new_ac.insert(s);
      });
    }
    ac = new_ac;
    rounds += 1;
    if ac.contains("e") {
      return rounds;
    }
  }
}

type Graph<T> = HashMap<T, HashSet<T>>;

fn parse(s: &str) -> (Graph<&str>, &str) {
  let mut lines = s.lines();
  let mut g = Graph::<&str>::default();
  for line in lines.by_ref() {
    if line.is_empty() {
      break;
    }
    let (lhs, rhs) = line.split_once(" => ").unwrap();
    g.entry(lhs).or_default().insert(rhs);
  }
  let target = lines.next().unwrap();
  assert!(lines.next().is_none());
  (g, target)
}

fn replace_all<F>(g: &Graph<&str>, start: &str, f: &mut F)
where
  F: FnMut(String),
{
  for (&k, vs) in g.iter() {
    for v in vs.iter() {
      let mut cursor = 0usize;
      while let Some(idx) = start.get(cursor..).and_then(|s| s.find(k)) {
        cursor += idx;
        // the `k` will be at the very front for the replacen.
        let replaced = format!("{}{}", &start[..cursor], start[cursor..].replacen(k, v, 1));
        f(replaced);
        cursor += 1;
      }
    }
  }
}

#[test]
fn t() {
  let s = include_str!("input/d19.txt");
  assert_eq!(p1(s), 509);
  // assert_eq!(p2(s), 0);
}

#[test]
fn ex1() {
  let s = "
H => HO
H => OH
O => HH

HOH
"
  .trim();
  assert_eq!(p1(s), 4);
}

#[test]
fn ex2() {
  let s = "
H => HO
H => OH
O => HH

HOHOHO
"
  .trim();
  assert_eq!(p1(s), 7);
}
