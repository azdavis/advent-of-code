fn get_stack(s: &str) -> Result<Vec<char>, usize> {
  let mut stack = Vec::<char>::default();
  for c in s.chars() {
    match c {
      '(' | '[' | '{' | '<' => stack.push(c),
      ')' => {
        if stack.pop() != Some('(') {
          return Err(3);
        }
      }
      ']' => {
        if stack.pop() != Some('[') {
          return Err(57);
        }
      }
      '}' => {
        if stack.pop() != Some('{') {
          return Err(1197);
        }
      }
      '>' => {
        if stack.pop() != Some('<') {
          return Err(25137);
        }
      }
      _ => panic!("unknown char: {}", c),
    }
  }
  Ok(stack)
}

pub fn p1(s: &str) -> usize {
  s.lines().filter_map(|line| get_stack(line).err()).sum()
}

pub fn p2(s: &str) -> usize {
  let mut scores: Vec<_> = s
    .lines()
    .filter_map(|line| {
      let stack = get_stack(line).ok()?;
      let score = stack.into_iter().rev().fold(0usize, |ac, c| {
        let add = match c {
          '(' => 1,
          '[' => 2,
          '{' => 3,
          '<' => 4,
          _ => panic!("unknown char: {}", c),
        };
        (ac * 5) + add
      });
      Some(score)
    })
    .collect();
  scores.sort_unstable();
  scores[scores.len() / 2]
}

#[test]
fn t() {
  let s = include_str!("input/d10.txt");
  assert_eq!(p1(s), 436497);
  assert_eq!(p2(s), 2377613374);
}
