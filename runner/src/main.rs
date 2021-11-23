fn main() {
  let s = include_str!("../../years/y2019/src/input/d23.txt");
  println!("p1: {}", y2019::d23::p1(s));
  println!("p2: {}", y2019::d23::p2(s));
}
