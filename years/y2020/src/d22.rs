use helpers::HashSet;
use std::cmp::Ordering;
use std::collections::VecDeque;

pub fn p1(s: &str) -> usize {
  let [d1, d2] = parse(s);
  score(regular(d1, d2))
}

pub fn p2(s: &str) -> usize {
  let [d1, d2] = parse(s);
  score(recursive(d1, d2).1)
}

type Card = usize;
type Deck = VecDeque<Card>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
  P1,
  P2,
}

fn regular(mut d1: Deck, mut d2: Deck) -> Deck {
  loop {
    match (d1.is_empty(), d2.is_empty()) {
      (true, true) => panic!("both decks empty"),
      (false, true) => return d1,
      (true, false) => return d2,
      (false, false) => {}
    }
    let c1 = d1.pop_front().unwrap();
    let c2 = d2.pop_front().unwrap();
    match c1.cmp(&c2) {
      Ordering::Less => {
        d2.push_back(c2);
        d2.push_back(c1);
      }
      Ordering::Equal => panic!("equal cards: {}", c1),
      Ordering::Greater => {
        d1.push_back(c1);
        d1.push_back(c2);
      }
    }
  }
}

fn recursive(mut d1: Deck, mut d2: Deck) -> (Player, Deck) {
  let mut prev = HashSet::default();
  loop {
    if !prev.insert((d1.clone(), d2.clone())) {
      return (Player::P1, d1);
    }
    match (d1.is_empty(), d2.is_empty()) {
      (true, true) => panic!("both decks empty"),
      (false, true) => return (Player::P1, d1),
      (true, false) => return (Player::P2, d2),
      (false, false) => {}
    }
    let c1 = d1.pop_front().unwrap();
    let c2 = d2.pop_front().unwrap();
    let winner = if d1.len() >= c1 && d2.len() >= c2 {
      let d1: VecDeque<_> = d1.iter().take(c1).copied().collect();
      let d2: VecDeque<_> = d2.iter().take(c2).copied().collect();
      recursive(d1, d2).0
    } else {
      match c1.cmp(&c2) {
        Ordering::Less => Player::P2,
        Ordering::Equal => panic!("equal cards: {}", c1),
        Ordering::Greater => Player::P1,
      }
    };
    match winner {
      Player::P1 => {
        d1.push_back(c1);
        d1.push_back(c2);
      }
      Player::P2 => {
        d2.push_back(c2);
        d2.push_back(c1);
      }
    }
  }
}

fn score(deck: Deck) -> usize {
  deck
    .into_iter()
    .rev()
    .enumerate()
    .map(|(idx, c)| (idx + 1) * c)
    .sum()
}

fn parse(s: &str) -> [Deck; 2] {
  let mut lines = s.lines();
  assert_eq!(lines.next().unwrap(), "Player 1:");
  let mut d1 = Deck::new();
  loop {
    let next = lines.next().unwrap();
    if next.is_empty() {
      break;
    }
    d1.push_back(next.parse().unwrap());
  }
  assert_eq!(lines.next().unwrap(), "Player 2:");
  let d2: Deck = lines.map(|x| x.parse().unwrap()).collect();
  [d1, d2]
}

#[test]
fn t() {
  let s = include_str!("input/d22.txt");
  assert_eq!(p1(s), 31957);
  assert_eq!(p2(s), 33212);
}

#[test]
fn t_p2_infinite() {
  let (p, _) = recursive(Deck::from(vec![43, 19]), Deck::from(vec![2, 29, 14]));
  assert_eq!(p, Player::P1);
}

#[test]
fn t_p2() {
  let (p, d) = recursive(
    Deck::from(vec![9, 2, 6, 3, 1]),
    Deck::from(vec![5, 8, 4, 7, 10]),
  );
  assert_eq!(p, Player::P2);
  assert_eq!(d, Deck::from(vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]));
}
