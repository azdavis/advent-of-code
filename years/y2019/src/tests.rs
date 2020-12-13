#[test]
fn d01() {
  let inp = include_str!("input/d01.txt");
  assert_eq!(crate::d01::p1(inp), 3296560);
  assert_eq!(crate::d01::p2(inp), 4941976);
}

#[test]
fn d02() {
  let inp = include_str!("input/d02.txt");
  assert_eq!(crate::d02::p1(inp), 11590668);
  assert_eq!(crate::d02::p2(inp), 2254);
}

#[test]
fn d03() {
  let inp = include_str!("input/d03.txt");
  assert_eq!(crate::d03::p1(inp), 1626);
  assert_eq!(crate::d03::p2(inp), 27330);
}
