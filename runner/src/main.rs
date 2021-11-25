fn main() {
  let s = include_str!("../../years/y2018/src/input/d02.txt");
  println!("p1: {}", y2018::d02::p1(s));
  println!("p2: {}", y2018::d02::p2(s));
}
