use helpers::maplit::hashset;
use std::collections::{HashMap, HashSet};

pub fn p1(s: &str) -> usize {
  let (rules, messages) = parse(s);
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

pub fn p2(s: &str) -> usize {
  todo!()
}

/// the messages are in reverse, and idx is an index into rules.
fn match_prefix(ms: &mut HashSet<Msg>, rules: &[Rule], idx: usize) {
  match rules[idx] {
    Rule::Char(c) => {
      *ms = ms
        .drain()
        .filter_map(|mut m| {
          if m.pop().map_or(false, |x| x == c) {
            Some(m)
          } else {
            None
          }
        })
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

fn match_prefix_seq(ms: &mut HashSet<Msg>, rules: &[Rule], seq: &[usize]) {
  for &idx in seq {
    match_prefix(ms, rules, idx);
  }
}

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

fn parse(s: &str) -> (Vec<Rule>, Vec<Msg>) {
  let mut lines = s.split('\n');
  let mut map = HashMap::new();
  loop {
    let line = lines.next().unwrap();
    if line.is_empty() {
      break;
    }
    let mut parts = line.split(": ");
    let num: usize = parts.next().unwrap().parse().unwrap();
    let rule = parse_rule(parts.next().unwrap());
    assert!(parts.next().is_none());
    map.insert(num, rule);
  }
  let mut rules = Vec::with_capacity(map.len());
  for i in 0..map.len() {
    let rule = map.remove(&i).unwrap();
    rules.push(rule);
  }
  let messages = lines
    .filter(|line| !line.is_empty())
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
  let inp = include_str!("input/d19.txt");
  assert_eq!(p1(inp), 144);
  // assert_eq!(p2(inp), ___);
}
