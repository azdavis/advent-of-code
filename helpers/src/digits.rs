//! Iterating over digits.

/// Returns an iterator over the decimal digits of `n` from left to right.
#[must_use]
pub fn get(n: u32) -> Digits {
  let mut div = 1u32;
  loop {
    let next = match div.checked_mul(10) {
      None => return Digits { div, n },
      Some(n) => n,
    };
    if next > n {
      return Digits { div, n };
    }
    div = next;
  }
}

/// An iterator over the decimal digits of a number from left to right.
#[derive(Debug)]
pub struct Digits {
  div: u32,
  n: u32,
}

impl Iterator for Digits {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.div == 0 {
      return None;
    }
    let ret = self.n / self.div;
    self.n -= ret * self.div;
    self.div /= 10;
    Some(ret)
  }
}

/// Returns the char corresponding to the digit `n`.
///
/// # Panics
///
/// If `n` is not a digit (i.e. if n >= 10).
#[must_use]
pub fn to_char(n: u32) -> char {
  match n {
    0 => '0',
    1 => '1',
    2 => '2',
    3 => '3',
    4 => '4',
    5 => '5',
    6 => '6',
    7 => '7',
    8 => '8',
    9 => '9',
    _ => panic!("not a digit: {n}"),
  }
}

#[cfg(test)]
mod tests {
  fn go(n: u32) -> Vec<u32> {
    super::get(n).collect()
  }

  #[test]
  fn t() {
    assert_eq!(go(0), vec![0]);
    assert_eq!(go(3), vec![3]);
    assert_eq!(go(10), vec![1, 0]);
    assert_eq!(go(123), vec![1, 2, 3]);
    assert_eq!(go(456_456), vec![4, 5, 6, 4, 5, 6]);
    assert_eq!(go(63_544), vec![6, 3, 5, 4, 4]);
  }
}
