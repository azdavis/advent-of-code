//! make sure we don't regress if we refactor

#[test]
fn t_01() {
  let inp = include_str!("input/01.txt");
  assert_eq!(crate::d01::p1(inp), 840324);
  assert_eq!(crate::d01::p2(inp), 170098110);
}

#[test]
fn t_02() {
  let inp = include_str!("input/02.txt");
  assert_eq!(crate::d02::p1(inp), 447);
  assert_eq!(crate::d02::p2(inp), 249);
}

#[test]
fn t_03() {
  let inp = include_str!("input/03.txt");
  assert_eq!(crate::d03::p1(inp), 289);
  assert_eq!(crate::d03::p2(inp), 5522401584);
}

#[test]
fn t_04() {
  let inp = include_str!("input/04.txt");
  assert_eq!(crate::d04::p1(inp), 233);
  assert_eq!(crate::d04::p2(inp), 111);
}

#[test]
fn t_05() {
  let inp = include_str!("input/05.txt");
  assert_eq!(crate::d05::p1(inp), 861);
  assert_eq!(crate::d05::p2(inp), 633);
}

#[test]
fn t_06() {
  let inp = include_str!("input/06.txt");
  assert_eq!(crate::d06::p1(inp), 6585);
  assert_eq!(crate::d06::p2(inp), 3276);
}

#[test]
fn t_07() {
  let inp = include_str!("input/07.txt");
  assert_eq!(crate::d07::p1(inp), 348);
  assert_eq!(crate::d07::p2(inp), 18885);
}

#[test]
fn t_08() {
  let inp = include_str!("input/08.txt");
  assert_eq!(crate::d08::p1(inp), 1709);
  assert_eq!(crate::d08::p2(inp), 1976);
}

#[test]
fn t_09() {
  let inp = include_str!("input/09.txt");
  assert_eq!(crate::d09::p1(inp), 675280050);
  assert_eq!(crate::d09::p2(inp), 96081673);
}

#[test]
fn t_10() {
  let inp = include_str!("input/10.txt");
  assert_eq!(crate::d10::p1(inp), 1700);
  assert_eq!(crate::d10::p2(inp), 12401793332096);
}

#[test]
fn t_11() {
  let inp = include_str!("input/11.txt");
  assert_eq!(crate::d11::p1(inp), 2238);
  assert_eq!(crate::d11::p2(inp), 2013);
}
