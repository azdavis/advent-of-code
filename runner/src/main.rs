fn main() {
  let inp = std::fs::read_to_string("input/input.txt").unwrap();
  println!("{}", y2020::d10::p1(&inp));
}
