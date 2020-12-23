pub fn p1(s: &str) -> String {
  p1_help(s, 100)
}

pub fn p2(s: &str) -> u32 {
  todo!()
}

fn p1_help(s: &str, rounds: usize) -> String {
  let mut cups = parse(s);
  go(&mut cups, rounds);
  let mut idx = cups.iter().position(|&c| c == 1).unwrap();
  let mut ret = String::with_capacity(cups.len() - 1);
  loop {
    idx += 1;
    if idx >= cups.len() {
      idx = 0;
    }
    if cups[idx] == 1 {
      return ret;
    }
    let c = match cups[idx] {
      0 => '0',
      1 => '1',
      2 => '2',
      3 => '3',
      4 => '4',
      5 => '5',
      6 => '6',
      7 => '7',
      8 => '8',
      9 => '9',
      n => panic!("not a digit: {}", n),
    };
    ret.push(c)
  }
}

fn go(cups: &mut Vec<u32>, rounds: usize) {
  assert!(cups.len() >= 5);
  let min_cup = *cups.iter().min().unwrap();
  let max_cup = *cups.iter().max().unwrap();
  let mut cur_idx = 0;
  for _ in 0..rounds {
    let mut pick_up = Vec::with_capacity(3);
    for _ in 0..3 {
      let mut rm_idx = cur_idx + 1;
      if rm_idx >= cups.len() {
        cur_idx -= 1;
        rm_idx = 0;
      }
      pick_up.push(cups.remove(rm_idx));
    }
    let mut dest_val = cups[cur_idx] - 1;
    loop {
      if dest_val < min_cup {
        dest_val = max_cup;
      }
      if !pick_up.contains(&dest_val) {
        break;
      }
      dest_val -= 1;
    }
    let mut dest_idx = cups.iter().position(|&c| c == dest_val).unwrap() + 1;
    if dest_idx >= cups.len() {
      dest_idx = 0;
    }
    cups.splice(dest_idx..dest_idx, pick_up);
    cur_idx += if dest_idx <= cur_idx { 4 } else { 1 };
    cur_idx %= cups.len();
  }
}

fn parse(s: &str) -> Vec<u32> {
  s.trim_end()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect()
}

#[test]
fn t() {
  let inp = include_str!("input/d23.txt");
  assert_eq!(p1(inp), "89372645");
  // assert_eq!(p2(inp), ___);
}

#[test]
fn t_p1() {
  assert_eq!(p1_help("389125467", 10), "92658374");
  assert_eq!(p1_help("389125467", 100), "67384529");
}
