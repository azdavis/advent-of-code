#[test]
fn t_01() {
  let inp = include_str!("input/d01.txt");
  assert_eq!(crate::d01::p1(inp), 3296560);
  assert_eq!(crate::d01::p2(inp), 4941976);
}
