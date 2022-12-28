fn go<F>(s: &str, f: &mut F) -> String
where
  F: FnMut(&mut Vec<Vec<char>>, usize, usize, usize),
{
  let mut lines = s.lines();
  let mut stacks = Vec::<Vec<char>>::new();
  for line in &mut lines {
    if line.is_empty() {
      break;
    }
    let line = line.as_bytes();
    for (idx, chunk) in line.chunks(4).enumerate() {
      let c = char::from(chunk[1]);
      if c.is_alphabetic() {
        stacks.resize_with(stacks.len().max(idx + 1), Vec::new);
        stacks[idx].push(c);
      }
    }
  }
  for stack in &mut stacks {
    stack.reverse();
  }
  for line in lines {
    let (count, from_to) = line
      .strip_prefix("move ")
      .unwrap()
      .split_once(" from ")
      .unwrap();
    let (from, to) = from_to.split_once(" to ").unwrap();
    let count = count.parse::<usize>().unwrap();
    let from = from.parse::<usize>().unwrap() - 1;
    let to = to.parse::<usize>().unwrap() - 1;
    f(&mut stacks, count, from, to);
  }
  stacks.iter().map(|stack| *stack.last().unwrap()).collect()
}

pub fn p1(s: &str) -> String {
  go(s, &mut |stacks, count, from, to| {
    for _ in 0..count {
      let c = stacks[from].pop().unwrap();
      stacks[to].push(c);
    }
  })
}

pub fn p2(s: &str) -> String {
  go(s, &mut |stacks, count, from, to| {
    let mut tmp = Vec::<char>::new();
    for _ in 0..count {
      let c = stacks[from].pop().unwrap();
      tmp.push(c);
    }
    for c in tmp.into_iter().rev() {
      stacks[to].push(c);
    }
  })
}

#[test]
fn t() {
  let s = include_str!("input/d05.txt");
  assert_eq!(p1(s), "LJSVLTWQM");
  assert_eq!(p2(s), "BRQWDBBJM");
}

#[test]
fn ex1() {
  let s = include_str!("input/d05-ex1.txt");
  assert_eq!(p1(s), "CMZ");
  assert_eq!(p2(s), "MCD");
}
