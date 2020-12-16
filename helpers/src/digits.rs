pub fn digits(n: u32) -> Digits {
  let mut div = 1;
  loop {
    let next = div * 10;
    if next > n {
      return Digits { div, n };
    }
    div = next;
  }
}

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

#[cfg(test)]
mod tests {
  fn go(n: u32) -> Vec<u32> {
    super::digits(n).collect()
  }

  #[test]
  fn t() {
    assert_eq!(go(0), vec![0]);
    assert_eq!(go(3), vec![3]);
    assert_eq!(go(123), vec![1, 2, 3]);
    assert_eq!(go(456456), vec![4, 5, 6, 4, 5, 6]);
    assert_eq!(go(63544), vec![6, 3, 5, 4, 4]);
  }
}
