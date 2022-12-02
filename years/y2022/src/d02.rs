#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
  Rock,
  Paper,
  Scissors,
}

impl Hand {
  fn score(self) -> u32 {
    match self {
      Hand::Rock => 1,
      Hand::Paper => 2,
      Hand::Scissors => 3,
    }
  }

  fn beat_by(self) -> Self {
    match self {
      Hand::Rock => Hand::Paper,
      Hand::Paper => Hand::Scissors,
      Hand::Scissors => Hand::Rock,
    }
  }

  fn beats(self) -> Self {
    match self {
      Hand::Paper => Hand::Rock,
      Hand::Scissors => Hand::Paper,
      Hand::Rock => Hand::Scissors,
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
  Lose,
  Tie,
  Win,
}

impl Outcome {
  fn score(self) -> u32 {
    match self {
      Outcome::Lose => 0,
      Outcome::Tie => 3,
      Outcome::Win => 6,
    }
  }
}

enum SecondColumn {
  X,
  Y,
  Z,
}

fn parse_line(line: &str) -> (Hand, SecondColumn) {
  let mut chars = line.chars();
  let fst = match chars.next().unwrap() {
    'A' => Hand::Rock,
    'B' => Hand::Paper,
    'C' => Hand::Scissors,
    c => panic!("unknown first col char: {c}"),
  };
  assert_eq!(chars.next().unwrap(), ' ');
  let snd = match chars.next().unwrap() {
    'X' => SecondColumn::X,
    'Y' => SecondColumn::Y,
    'Z' => SecondColumn::Z,
    c => panic!("unknown second col char: {c}"),
  };
  assert!(chars.next().is_none());
  (fst, snd)
}

fn get<F>(s: &str, mut f: F) -> u32
where
  F: FnMut(Hand, SecondColumn) -> (Hand, Outcome),
{
  s.lines()
    .map(|line| {
      let (opponent, snd) = parse_line(line);
      let (me, outcome) = f(opponent, snd);
      me.score() + outcome.score()
    })
    .sum()
}

pub fn p1(s: &str) -> u32 {
  get(s, |opponent, snd| {
    let me = match snd {
      SecondColumn::X => Hand::Rock,
      SecondColumn::Y => Hand::Paper,
      SecondColumn::Z => Hand::Scissors,
    };
    let outcome = if me.beat_by() == opponent {
      Outcome::Lose
    } else if me == opponent {
      Outcome::Tie
    } else {
      Outcome::Win
    };
    (me, outcome)
  })
}

pub fn p2(s: &str) -> u32 {
  get(s, |opponent, snd| {
    let outcome = match snd {
      SecondColumn::X => Outcome::Lose,
      SecondColumn::Y => Outcome::Tie,
      SecondColumn::Z => Outcome::Win,
    };
    let me = match outcome {
      Outcome::Lose => opponent.beats(),
      Outcome::Tie => opponent,
      Outcome::Win => opponent.beat_by(),
    };
    (me, outcome)
  })
}

#[test]
fn t() {
  let s = include_str!("input/d02.txt");
  assert_eq!(p1(s), 12740);
  assert_eq!(p2(s), 11980);
}
