fn decode_len(s: &str) -> usize {
  let mut iter = s.chars();
  let mut ret = 0usize;
  assert_eq!(iter.next().unwrap(), '"');
  loop {
    match iter.next().unwrap() {
      '"' => {
        assert!(iter.next().is_none());
        return ret;
      }
      '\\' => match iter.next().unwrap() {
        '"' | '\\' => {}
        'x' => {
          assert!(iter.next().unwrap().is_ascii_hexdigit());
          assert!(iter.next().unwrap().is_ascii_hexdigit());
        }
        c => panic!("unknown escape: {c}"),
      },
      _ => {}
    }
    ret += 1;
  }
}

pub fn p1(s: &str) -> usize {
  let mut code = 0usize;
  let mut decode = 0usize;
  for line in s.lines() {
    code += line.len();
    decode += decode_len(line);
  }
  code - decode
}

fn encode_len(s: &str) -> usize {
  let sum: usize = s
    .chars()
    .map(|c| match c {
      '"' | '\\' => 2,
      _ => 1,
    })
    .sum();
  sum + 2
}

pub fn p2(s: &str) -> usize {
  let mut code = 0usize;
  let mut encode = 0usize;
  for line in s.lines() {
    code += line.len();
    encode += encode_len(line);
  }
  encode - code
}

#[test]
fn t() {
  let s = include_str!("input/d08.txt");
  assert_eq!(p1(s), 1350);
  assert_eq!(p2(s), 2085);
}
