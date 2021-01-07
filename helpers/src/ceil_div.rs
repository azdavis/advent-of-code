//! Integer ceiling division.

/// Returns `ceil(a / b)`.
pub fn ceil_div(a: usize, b: usize) -> usize {
  let mut ret = a / b;
  if a % b != 0 {
    ret += 1;
  }
  ret
}

#[test]
fn t() {
  assert_eq!(ceil_div(5, 1), 5);
  assert_eq!(ceil_div(2, 1), 2);
  assert_eq!(ceil_div(6, 2), 3);
  assert_eq!(ceil_div(5, 2), 3);
  assert_eq!(ceil_div(4, 2), 2);
  assert_eq!(ceil_div(5, 3), 2);
  assert_eq!(ceil_div(3, 3), 1);
  assert_eq!(ceil_div(2, 3), 1);
  assert_eq!(ceil_div(0, 3), 0);
}
