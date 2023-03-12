//! Calculating the GCD (and LCM).

/// Returns the greatest common divisor of `a` and `b` using Euclid's algorithm.
///
/// # Panics
///
/// If both are zero.
#[must_use]
pub fn gcd(mut a: usize, mut b: usize) -> usize {
  assert!(a != 0 || b != 0);
  while b != 0 {
    let tmp = b;
    b = a % b;
    a = tmp;
  }
  a
}

/// Returns the least common multiple of `a` and `b`. Panics if both are zero.
#[must_use]
pub fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
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
