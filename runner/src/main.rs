fn main() {
  let s = include_str!("../../years/y2018/src/input/d05.txt");
  println!("p1: {}", y2018::d05::p1(s));
  println!("p2: {}", y2018::d05::p2(s));
}
