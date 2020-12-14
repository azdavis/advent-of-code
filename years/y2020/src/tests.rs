//! make sure we don't regress if we refactor

#[test]
fn d01() {
  let inp = include_str!("input/d01.txt");
  assert_eq!(crate::d01::p1(inp), 840324);
  assert_eq!(crate::d01::p2(inp), 170098110);
}

#[test]
fn d02() {
  let inp = include_str!("input/d02.txt");
  assert_eq!(crate::d02::p1(inp), 447);
  assert_eq!(crate::d02::p2(inp), 249);
}

#[test]
fn d03() {
  let inp = include_str!("input/d03.txt");
  assert_eq!(crate::d03::p1(inp), 289);
  assert_eq!(crate::d03::p2(inp), 5522401584);
}

#[test]
fn d04() {
  let inp = include_str!("input/d04.txt");
  assert_eq!(crate::d04::p1(inp), 233);
  assert_eq!(crate::d04::p2(inp), 111);
}

#[test]
fn d05() {
  let inp = include_str!("input/d05.txt");
  assert_eq!(crate::d05::p1(inp), 861);
  assert_eq!(crate::d05::p2(inp), 633);
}

#[test]
fn d06() {
  let inp = include_str!("input/d06.txt");
  assert_eq!(crate::d06::p1(inp), 6585);
  assert_eq!(crate::d06::p2(inp), 3276);
}

#[test]
fn d07() {
  let inp = include_str!("input/d07.txt");
  assert_eq!(crate::d07::p1(inp), 348);
  assert_eq!(crate::d07::p2(inp), 18885);
}

#[test]
fn d08() {
  let inp = include_str!("input/d08.txt");
  assert_eq!(crate::d08::p1(inp), 1709);
  assert_eq!(crate::d08::p2(inp), 1976);
}

#[test]
fn d09() {
  let inp = include_str!("input/d09.txt");
  assert_eq!(crate::d09::p1(inp), 675280050);
  assert_eq!(crate::d09::p2(inp), 96081673);
}

#[test]
fn d10() {
  let inp = include_str!("input/d10.txt");
  assert_eq!(crate::d10::p1(inp), 1700);
  assert_eq!(crate::d10::p2(inp), 12401793332096);
}

#[test]
fn d11() {
  let inp = include_str!("input/d11.txt");
  assert_eq!(crate::d11::p1(inp), 2238);
  assert_eq!(crate::d11::p2(inp), 2013);
}

#[test]
fn d12() {
  let inp = include_str!("input/d12.txt");
  assert_eq!(crate::d12::p1(inp), 1133);
  assert_eq!(crate::d12::p2(inp), 61053);
}

#[test]
fn d13() {
  let inp = include_str!("input/d13.txt");
  assert_eq!(crate::d13::p1(inp), 333);
  assert_eq!(crate::d13::p2(inp), 690123192779524);
}

#[test]
fn d14() {
  let inp = include_str!("input/d14.txt");
  assert_eq!(crate::d14::p1(inp), 11179633149677);
}
