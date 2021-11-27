fn main() {
  let s = include_str!("../../years/y2018/src/input/d06.txt");
  println!("p1: {}", y2018::d06::p1(s));
  println!("p2: {}", y2018::d06::p2(s));
}
