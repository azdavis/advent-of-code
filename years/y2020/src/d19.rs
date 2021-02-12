use helpers::maplit::hashset;
use std::collections::{HashMap, HashSet};

pub fn p1(s: &str) -> usize {
  let (rules, messages) = parse(s);
  go(rules, messages)
}

pub fn p2(s: &str) -> usize {
  let (mut rules, messages) = parse(s);
  rules.insert(8, Rule::Alt(vec![42], vec![42, 8]));
  rules.insert(11, Rule::Alt(vec![42, 31], vec![42, 11, 31]));
  go(rules, messages)
}

fn go(rules: Rules, messages: Vec<Msg>) -> usize {
  let mut ret = 0;
  for mut m in messages {
    // don't use filter + count since we mutate
    m.reverse();
    let mut ms = hashset![m];
    match_prefix(&mut ms, &rules, 0);
    if ms.contains(&vec![]) {
      ret += 1;
    }
  }
  ret
}

/// the messages are in reverse, and idx is an index into rules.
fn match_prefix(ms: &mut HashSet<Msg>, rules: &Rules, idx: usize) {
  if ms.is_empty() {
    return;
  }
  match *rules.get(&idx).unwrap() {
    Rule::Char(c) => {
      *ms = ms
        .drain()
        .filter_map(|mut m| m.pop().map_or(false, |x| x == c).then(|| m))
        .collect();
    }
    Rule::Seq(ref seq) => match_prefix_seq(ms, rules, seq),
    Rule::Alt(ref a, ref b) => {
      let mut other = ms.clone();
      match_prefix_seq(ms, rules, a);
      match_prefix_seq(&mut other, rules, b);
      ms.extend(other);
    }
  }
}

fn match_prefix_seq(ms: &mut HashSet<Msg>, rules: &Rules, seq: &[usize]) {
  for &idx in seq {
    match_prefix(ms, rules, idx);
  }
}

type Rules = HashMap<usize, Rule>;

enum Rule {
  Char(Char),
  Seq(Vec<usize>),
  Alt(Vec<usize>, Vec<usize>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Char {
  A,
  B,
}

type Msg = Vec<Char>;

fn parse(s: &str) -> (Rules, Vec<Msg>) {
  let mut lines = s.lines();
  let mut rules = HashMap::new();
  loop {
    let line = lines.next().unwrap();
    if line.is_empty() {
      break;
    }
    let mut parts = line.split(": ");
    let num: usize = parts.next().unwrap().parse().unwrap();
    let rule = parse_rule(parts.next().unwrap());
    assert!(parts.next().is_none());
    rules.insert(num, rule);
  }
  let messages = lines
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          'a' => Char::A,
          'b' => Char::B,
          _ => panic!("bad char: {}", c),
        })
        .collect()
    })
    .collect();
  (rules, messages)
}

fn parse_rule(s: &str) -> Rule {
  let mut parts = s.split(' ');
  let fst: usize = match parts.next().unwrap() {
    "\"a\"" => return Rule::Char(Char::A),
    "\"b\"" => return Rule::Char(Char::B),
    x => x.parse().unwrap(),
  };
  let mut cur = vec![fst];
  let mut fst_alt: Option<Vec<usize>> = None;
  for s in parts {
    if s == "|" {
      assert!(fst_alt.is_none());
      fst_alt = Some(cur);
      cur = vec![];
    } else {
      let n: usize = s.parse().unwrap();
      cur.push(n);
    }
  }
  match fst_alt {
    None => Rule::Seq(cur),
    Some(x) => Rule::Alt(x, cur),
  }
}

#[test]
fn t() {
  let s = include_str!("input/d19.txt");
  assert_eq!(p1(s), 144);
  assert_eq!(p2(s), 260);
}
