fn main() {
  let inp = std::fs::read_to_string("input/input.txt").unwrap();
  println!("{}", y2020::d11::p2(&inp));
}
