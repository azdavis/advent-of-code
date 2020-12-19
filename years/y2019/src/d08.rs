pub fn p1(s: &str) -> usize {
  let ns = parse(s);
  let least_zeroes = ns
    .chunks(WIDTH * HEIGHT)
    .min_by_key(|xs| xs.iter().filter(|&&x| x == 0).count())
    .unwrap();
  let num_ones = least_zeroes.iter().filter(|&&x| x == 1).count();
  let num_twos = least_zeroes.iter().filter(|&&x| x == 2).count();
  num_ones * num_twos
}

pub fn p2(s: &str) -> u32 {
  todo!()
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn parse(s: &str) -> Vec<u32> {
  s.split('\n')
    .next()
    .unwrap()
    .chars()
    .map(|x| x.to_digit(10).unwrap())
    .collect()
}

#[test]
fn t() {
  let inp = include_str!("input/d08.txt");
  assert_eq!(p1(inp), 2760);
  // assert_eq!(p2(inp), ___);
}
