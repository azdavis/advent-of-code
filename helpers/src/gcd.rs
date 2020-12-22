pub fn gcd(mut a: usize, mut b: usize) -> usize {
  assert!(a != 0 || b != 0);
  while b != 0 {
    let tmp = b;
    b = a % b;
    a = tmp;
  }
  a
}

#[test]
fn t() {
  assert_eq!(gcd(4, 0), 4);
  assert_eq!(gcd(4, 1), 1);
  assert_eq!(gcd(2, 5), 1);
  assert_eq!(gcd(2, 10), 2);
  assert_eq!(gcd(3, 6), 3);
  assert_eq!(gcd(4, 6), 2);
}
