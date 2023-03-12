use helpers::{static_regex, HashMap};
use std::str::FromStr;

static_regex!(BOT = r"^bot (\d+) gives low to (\w+) (\d+) and high to (\w+) (\d+)$");

static_regex!(VALUE = r"^value (\d+) goes to bot (\d+)$");

#[derive(Debug, Clone, Copy)]
enum Give {
  Bot,
  Out,
}

impl FromStr for Give {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let ret = match s {
      "bot" => Give::Bot,
      "output" => Give::Out,
      _ => return Err(()),
    };
    Ok(ret)
  }
}

type Rule = [(Give, u16); 2];
type Rules = HashMap<u16, Rule>;
type Ones = HashMap<u16, u16>;
type Twos = HashMap<u16, [u16; 2]>;

fn add_val(ones: &mut Ones, twos: &mut Twos, bot: u16, val: u16) {
  match ones.remove(&bot) {
    Some(other) => {
      let mut ins = [other, val];
      ins.sort_unstable();
      assert!(twos.insert(bot, ins).is_none());
    }
    None => assert!(ones.insert(bot, val).is_none()),
  }
}

fn parse(s: &str) -> (Rules, Ones, Twos) {
  let mut rules = Rules::default();
  let mut ones = Ones::default();
  let mut twos = Twos::default();
  for line in s.lines() {
    if let Some(caps) = BOT.captures(line) {
      let bot: u16 = caps[1].parse().unwrap();
      let rule: Rule = [
        (caps[2].parse().unwrap(), caps[3].parse().unwrap()),
        (caps[4].parse().unwrap(), caps[5].parse().unwrap()),
      ];
      rules.insert(bot, rule);
    } else if let Some(caps) = VALUE.captures(line) {
      let val: u16 = caps[1].parse().unwrap();
      let bot: u16 = caps[2].parse().unwrap();
      add_val(&mut ones, &mut twos, bot, val);
    } else {
      panic!("cannot parse line: {line}");
    }
  }
  (rules, ones, twos)
}

type Out = HashMap<u16, u16>;

fn do_give(out: &mut Out, ones: &mut Ones, twos: &mut Twos, give: Give, idx: u16, val: u16) {
  match give {
    Give::Bot => add_val(ones, twos, idx, val),
    Give::Out => assert!(out.insert(idx, val).is_none()),
  }
}

fn run(s: &str) -> (u16, u16) {
  let (rules, mut ones, mut twos) = parse(s);
  let mut out = Out::default();
  let mut p1 = None::<u16>;
  while !twos.is_empty() {
    let mut new_twos = Twos::default();
    for (bot, [lo_v, hi_v]) in twos {
      if lo_v == 17 && hi_v == 61 {
        p1 = Some(bot);
      }
      let &[(lo_give, lo_idx), (hi_give, hi_idx)] = &rules[&bot];
      do_give(&mut out, &mut ones, &mut new_twos, lo_give, lo_idx, lo_v);
      do_give(&mut out, &mut ones, &mut new_twos, hi_give, hi_idx, hi_v);
    }
    twos = new_twos;
  }
  let p2 = out[&0] * out[&1] * out[&2];
  (p1.unwrap(), p2)
}

pub fn p1(s: &str) -> u16 {
  run(s).0
}

pub fn p2(s: &str) -> u16 {
  run(s).1
}

#[test]
fn t() {
  let s = include_str!("input/d10.txt");
  assert_eq!(p1(s), 141);
  assert_eq!(p2(s), 1209);
}
