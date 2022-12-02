#[derive(Debug, Clone, Copy)]
enum Hand {
  Rock,
  Paper,
  Scissors,
}

pub fn p1(s: &str) -> u32 {
  s.lines()
    .map(|line| {
      let mut chars = line.chars();
      let opponent = match chars.next().unwrap() {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        'C' => Hand::Scissors,
        c => panic!("unknown first col char: {c}"),
      };
      assert_eq!(chars.next().unwrap(), ' ');
      let me = match chars.next().unwrap() {
        'X' => Hand::Rock,
        'Y' => Hand::Paper,
        'Z' => Hand::Scissors,
        c => panic!("unknown second col char: {c}"),
      };
      assert!(chars.next().is_none());
      let shape_score: u32 = match me {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
      };
      let outcome_score: u32 = match (me, opponent) {
        (Hand::Rock, Hand::Rock)
        | (Hand::Paper, Hand::Paper)
        | (Hand::Scissors, Hand::Scissors) => 3,
        (Hand::Rock, Hand::Scissors)
        | (Hand::Paper, Hand::Rock)
        | (Hand::Scissors, Hand::Paper) => 6,
        (Hand::Scissors, Hand::Rock)
        | (Hand::Rock, Hand::Paper)
        | (Hand::Paper, Hand::Scissors) => 0,
      };
      shape_score + outcome_score
    })
    .sum()
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  let s = include_str!("input/d02.txt");
  assert_eq!(p1(s), 12740);
  assert_eq!(p2(s), 0);
}
